mod common;

use reqwest::blocking::Client;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

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
