mod common;

use reqwest::blocking::Client;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::net::TcpStream;
use std::time::{Duration, Instant};

#[test]
fn fragmented_push_request_is_accepted_and_pullable() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let addr = server
        .base_url()
        .strip_prefix("http://")
        .expect("http base url");

    let channel = "na0173fragchan01";
    let payload = b"hello";
    let req = format!(
        "POST /v1/push/{channel} HTTP/1.1\r\nHost: {addr}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    );

    let mut stream = TcpStream::connect(addr).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("set timeout");
    stream.set_nodelay(true).expect("nodelay");

    // Force header/body fragmentation across writes.
    stream
        .write_all(&req.as_bytes()[..16])
        .expect("write header part 1");
    stream
        .write_all(&req.as_bytes()[16..])
        .expect("write header part 2");
    stream.write_all(payload).expect("write body");
    stream.flush().expect("flush");

    let mut resp = Vec::new();
    stream.read_to_end(&mut resp).expect("read response");
    let text = String::from_utf8_lossy(&resp);
    assert!(
        text.starts_with("HTTP/1.1 200"),
        "expected 200 response for fragmented request, got: {text}"
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("build client");
    let pull_url = format!("{}/v1/pull/{}?max=1", server.base_url(), channel);
    let pull = client.get(&pull_url).send().expect("pull request");
    assert_eq!(pull.status().as_u16(), 200, "pull status");
    let body = pull.text().expect("pull body");
    assert!(
        body.contains("\"items\":[{"),
        "pull body missing item: {body}"
    );
}

fn connect_with_timeout(addr: &str) -> TcpStream {
    let stream = TcpStream::connect(addr).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(4)))
        .expect("set timeout");
    stream.set_nodelay(true).expect("nodelay");
    stream
}

fn read_http_response(mut stream: TcpStream) -> String {
    let mut resp = Vec::new();
    stream.read_to_end(&mut resp).expect("read response");
    String::from_utf8_lossy(&resp).to_string()
}

#[test]
fn conflicting_content_length_is_rejected_and_not_enqueued() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let addr = server
        .base_url()
        .strip_prefix("http://")
        .expect("http base url");
    let channel = "na0174conflictcl01";
    let payload = b"abcde";

    let req = format!(
        "POST /v1/push/{channel} HTTP/1.1\r\nHost: {addr}\r\nContent-Length: 5\r\nContent-Length: 7\r\nConnection: close\r\n\r\n"
    );

    let mut stream = connect_with_timeout(addr);
    stream
        .write_all(req.as_bytes())
        .expect("write conflicting header");
    stream.write_all(payload).expect("write body bytes");
    stream.flush().expect("flush");
    let text = read_http_response(stream);

    assert!(
        text.starts_with("HTTP/1.1 400"),
        "expected deterministic 400 on conflicting content-length, got: {text}"
    );

    // Ensure failed push does not enqueue anything.
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("build client");
    let pull_url = format!("{}/v1/pull/{}?max=1", server.base_url(), channel);
    let pull = client.get(&pull_url).send().expect("pull request");
    assert_eq!(
        pull.status().as_u16(),
        204,
        "queue should remain empty after conflicting content-length reject"
    );
}

#[test]
fn truncated_body_is_rejected_within_bounded_deadline() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let addr = server
        .base_url()
        .strip_prefix("http://")
        .expect("http base url");
    let channel = "na0174truncbody01";
    let declared_len = 32usize;
    let sent = b"short";

    let req = format!(
        "POST /v1/push/{channel} HTTP/1.1\r\nHost: {addr}\r\nContent-Length: {declared_len}\r\nConnection: close\r\n\r\n"
    );

    let mut stream = connect_with_timeout(addr);
    stream.write_all(req.as_bytes()).expect("write header");
    stream.write_all(sent).expect("write partial body");
    stream.flush().expect("flush");
    stream
        .shutdown(Shutdown::Write)
        .expect("shutdown write side");

    let start = Instant::now();
    let text = read_http_response(stream);
    let elapsed = start.elapsed();

    assert!(
        elapsed <= Duration::from_secs(4),
        "truncated-body handling exceeded bounded deadline: {elapsed:?}"
    );
    assert!(
        text.starts_with("HTTP/1.1 400"),
        "expected deterministic 400 on truncated body, got: {text}"
    );

    // Ensure failed push does not enqueue anything.
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("build client");
    let pull_url = format!("{}/v1/pull/{}?max=1", server.base_url(), channel);
    let pull = client.get(&pull_url).send().expect("pull request");
    assert_eq!(
        pull.status().as_u16(),
        204,
        "queue should remain empty after truncated-body reject"
    );
}
