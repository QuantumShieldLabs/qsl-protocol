use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const ROUTE_TOKEN: &str = "na0217g_route_token_abcdefghijklmnop";

#[derive(Deserialize)]
struct PullItem {
    id: String,
    data: Vec<u8>,
}

#[derive(Deserialize)]
struct PullResp {
    items: Vec<PullItem>,
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn absolute_test_root(tag: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join("qsc-test-tmp").join(format!(
        "{tag}_{}_{}",
        std::process::id(),
        stamp
    ));
    create_dir_700(&root);
    root
}

struct LocalRelay {
    child: Child,
    log_path: PathBuf,
}

impl LocalRelay {
    fn start(root: &Path) -> (Self, String) {
        let log_path = root.join("relay.log");
        let log = fs::File::create(&log_path).expect("relay log");
        let child = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
            .env("QSC_MARK_FORMAT", "plain")
            .args([
                "relay",
                "serve",
                "--port",
                "0",
                "--seed",
                "0",
                "--drop-pct",
                "0",
                "--dup-pct",
                "0",
                "--reorder-window",
                "0",
                "--fixed-latency-ms",
                "0",
                "--jitter-ms",
                "0",
            ])
            .stdout(Stdio::from(log))
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn relay");
        let mut relay = Self { child, log_path };
        let deadline = Instant::now() + Duration::from_secs(5);
        let port = loop {
            let text = fs::read_to_string(&relay.log_path).unwrap_or_default();
            if let Some(port) = text
                .lines()
                .find_map(|line| line.split("event=relay_listen port=").nth(1))
                .and_then(|tail| tail.split_whitespace().next())
            {
                break port.to_string();
            }
            if let Some(status) = relay.child.try_wait().expect("poll relay child") {
                panic!(
                    "relay exited before readiness: status={status} log={}",
                    fs::read_to_string(&relay.log_path).unwrap_or_default()
                );
            }
            assert!(Instant::now() < deadline, "relay did not become ready");
            thread::sleep(Duration::from_millis(20));
        };
        (relay, format!("http://127.0.0.1:{port}"))
    }
}

impl Drop for LocalRelay {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("build client")
}

fn push_payload(client: &Client, relay_url: &str, payload: &[u8]) {
    let resp = client
        .post(format!("{relay_url}/v1/push"))
        .header("X-QSL-Route-Token", ROUTE_TOKEN)
        .body(payload.to_vec())
        .send()
        .expect("push request");
    assert_eq!(resp.status().as_u16(), 200, "push status");
}

fn pull_items(client: &Client, relay_url: &str, max: usize) -> (u16, Vec<PullItem>) {
    let resp = client
        .get(format!("{relay_url}/v1/pull?max={max}"))
        .header("X-QSL-Route-Token", ROUTE_TOKEN)
        .send()
        .expect("pull request");
    let status = resp.status().as_u16();
    if status == 204 {
        return (status, Vec::new());
    }
    assert_eq!(status, 200, "pull status");
    let body: PullResp = resp.json().expect("pull body");
    (status, body.items)
}

#[test]
fn relay_transport_http_send_receive_is_fifo_and_bounded() {
    let root = absolute_test_root("na0217g_transport_contract");
    let (_relay, relay_url) = LocalRelay::start(&root);
    let client = http_client();

    push_payload(&client, &relay_url, b"alpha");
    push_payload(&client, &relay_url, b"beta");

    let (first_status, first_items) = pull_items(&client, &relay_url, 1);
    assert_eq!(
        first_status, 200,
        "first pull should return one queued item"
    );
    assert_eq!(first_items.len(), 1, "max=1 must bound first pull");
    assert_eq!(first_items[0].data, b"alpha");
    assert!(
        !first_items[0].id.is_empty(),
        "first item id must be populated"
    );

    let (second_status, second_items) = pull_items(&client, &relay_url, 1);
    assert_eq!(
        second_status, 200,
        "second pull should return the remaining item"
    );
    assert_eq!(second_items.len(), 1, "max=1 must bound second pull");
    assert_eq!(second_items[0].data, b"beta");
    assert!(
        second_items[0].id != first_items[0].id,
        "queued items should preserve distinct delivery ids"
    );

    let (third_status, third_items) = pull_items(&client, &relay_url, 1);
    assert_eq!(
        third_status, 204,
        "queue should be empty after two bounded pulls"
    );
    assert!(third_items.is_empty(), "204 pull must not carry items");
}
