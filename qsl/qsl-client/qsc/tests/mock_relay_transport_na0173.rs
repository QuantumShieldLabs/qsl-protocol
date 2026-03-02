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

fn read_http_response_from_stream(stream: &mut TcpStream) -> String {
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

#[test]
fn reject_transfer_encoding_chunked_on_push_and_not_enqueued() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let addr = server
        .base_url()
        .strip_prefix("http://")
        .expect("http base url");
    let channel = "na0175chunkedpush01";
    let req = format!(
        "POST /v1/push/{channel} HTTP/1.1\r\nHost: {addr}\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
    );
    let chunked_body = b"5\r\nhello\r\n0\r\n\r\n";

    let mut push_stream = connect_with_timeout(addr);
    push_stream
        .write_all(req.as_bytes())
        .expect("write chunked header");
    push_stream
        .write_all(chunked_body)
        .expect("write chunked body");
    push_stream.flush().expect("flush push");
    let push_text = read_http_response(push_stream);
    assert!(
        push_text.starts_with("HTTP/1.1 400"),
        "chunked push must be rejected deterministically, got: {push_text}"
    );

    let pull_req = format!(
        "GET /v1/pull/{channel}?max=1 HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n"
    );
    let mut pull_stream = connect_with_timeout(addr);
    pull_stream
        .write_all(pull_req.as_bytes())
        .expect("write pull request");
    pull_stream.flush().expect("flush pull");
    let pull_text = read_http_response(pull_stream);
    assert!(
        pull_text.starts_with("HTTP/1.1 204"),
        "chunked reject must not enqueue payload, got: {pull_text}"
    );
}

#[test]
fn second_request_on_same_connection_is_closed_or_rejected_bounded() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let addr = server
        .base_url()
        .strip_prefix("http://")
        .expect("http base url");
    let channel = "na0175secondreq01";

    let mut stream = connect_with_timeout(addr);
    let first_req = format!(
        "GET /v1/pull/{channel}?max=1 HTTP/1.1\r\nHost: {addr}\r\nConnection: keep-alive\r\n\r\n"
    );
    stream
        .write_all(first_req.as_bytes())
        .expect("write first request");
    stream.flush().expect("flush first request");
    let first_resp = read_http_response_from_stream(&mut stream);
    assert!(
        first_resp.starts_with("HTTP/1.1 204"),
        "first request should complete deterministically, got: {first_resp}"
    );

    let second_req = format!(
        "GET /v1/pull/{channel}?max=1 HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n"
    );
    let start = Instant::now();
    let write_res = stream.write_all(second_req.as_bytes());
    let elapsed = start.elapsed();
    assert!(
        elapsed <= Duration::from_secs(4),
        "second-request handling exceeded bounded deadline: {elapsed:?}"
    );

    if write_res.is_ok() {
        let mut second_resp = Vec::new();
        let _ = stream.read_to_end(&mut second_resp);
        let text = String::from_utf8_lossy(&second_resp);
        assert!(
            text.is_empty() || text.starts_with("HTTP/1.1 400"),
            "second request on same connection must be closed or rejected, got: {text}"
        );
    }
}
