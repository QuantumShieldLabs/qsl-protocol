use std::collections::BTreeMap;
use std::net::IpAddr;

pub const QSC_ERR_ROUTE_TOKEN_INVALID: &str = "QSC_ERR_ROUTE_TOKEN_INVALID";
pub const QSC_ERR_RELAY_TLS_REQUIRED: &str = "QSC_ERR_RELAY_TLS_REQUIRED";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpRelayTarget {
    Push,
    Pull(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpRequestParsed {
    pub method: String,
    pub target: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

pub fn route_token_is_valid(token: &str) -> bool {
    let trimmed = token.trim();
    !trimmed.is_empty()
        && (22..=128).contains(&trimmed.len())
        && trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub fn normalize_route_token(raw: &str) -> Result<String, &'static str> {
    let token = raw.trim();
    if route_token_is_valid(token) {
        Ok(token.to_string())
    } else {
        Err(QSC_ERR_ROUTE_TOKEN_INVALID)
    }
}

pub fn relay_host_is_loopback(host: &str) -> bool {
    let canonical = host.trim_matches(|c| c == '[' || c == ']');
    if canonical.eq_ignore_ascii_case("localhost") {
        return true;
    }
    canonical
        .parse::<IpAddr>()
        .map(|ip| ip.is_loopback())
        .unwrap_or(false)
}

pub fn validate_relay_endpoint_url(raw: &str) -> Result<reqwest::Url, &'static str> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("relay_endpoint_missing");
    }
    let parsed = reqwest::Url::parse(trimmed).map_err(|_| "relay_endpoint_invalid")?;
    let host = parsed.host_str().ok_or("relay_endpoint_invalid_host")?;
    let scheme = parsed.scheme();
    match scheme {
        "https" => Ok(parsed),
        "http" => {
            if relay_host_is_loopback(host) {
                Ok(parsed)
            } else {
                Err(QSC_ERR_RELAY_TLS_REQUIRED)
            }
        }
        _ => Err("relay_endpoint_invalid_scheme"),
    }
}

pub fn normalize_relay_endpoint(value: &str) -> Result<String, &'static str> {
    let parsed = validate_relay_endpoint_url(value)?;
    Ok(parsed.to_string().trim_end_matches('/').to_string())
}

pub fn relay_probe_url(endpoint: &str) -> Result<String, &'static str> {
    let endpoint = normalize_relay_endpoint(endpoint)?;
    Ok(format!("{}/v1/pull?max=1", endpoint.trim_end_matches('/')))
}

pub fn parse_http_target(target: &str) -> Option<HttpRelayTarget> {
    let (path, query) = match target.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (target, None),
    };
    if path == "/v1/push" {
        return Some(HttpRelayTarget::Push);
    }
    if path == "/v1/pull" {
        let mut max = 1usize;
        if let Some(query) = query {
            for part in query.split('&') {
                if let Some(raw) = part.strip_prefix("max=") {
                    if let Ok(parsed) = raw.parse::<usize>() {
                        max = parsed;
                    }
                }
            }
        }
        return Some(HttpRelayTarget::Pull(max));
    }
    None
}

pub fn parse_http_route_token(headers: &BTreeMap<String, String>) -> Result<String, &'static str> {
    let header_token = match headers.get("x-qsl-route-token") {
        None => None,
        Some(raw) => {
            let token = raw.trim();
            if token.is_empty() {
                return Err("missing_route_token");
            }
            Some(normalize_route_token(token).map_err(|_| "invalid_route_token")?)
        }
    };
    header_token.ok_or("missing_route_token")
}

pub fn parse_http_route_token_from_request(
    req: &HttpRequestParsed,
) -> Result<String, &'static str> {
    parse_http_route_token(&req.headers)
}

pub fn find_http_header_end(buf: &[u8]) -> Option<usize> {
    if buf.len() < 4 {
        return None;
    }
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

pub fn parse_http_request_bytes(bytes: &[u8]) -> Result<HttpRequestParsed, &'static str> {
    let header_end = find_http_header_end(bytes).ok_or("http_request_incomplete")?;
    let header_bytes = &bytes[..header_end];
    let header_text = std::str::from_utf8(header_bytes).map_err(|_| "http_request_invalid")?;
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().ok_or("http_request_invalid")?;
    let mut req = request_line.split_whitespace();
    let method = req.next().ok_or("http_request_invalid")?.to_string();
    let target = req.next().ok_or("http_request_invalid")?.to_string();
    let _http_version = req.next().ok_or("http_request_invalid")?;
    let mut headers = BTreeMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (k, v) = line.split_once(':').ok_or("http_request_invalid")?;
        headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
    }
    let body_start = header_end + 4;
    let content_len = headers
        .get("content-length")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    if bytes.len() < body_start.saturating_add(content_len) {
        return Err("http_request_incomplete");
    }
    let body = bytes[body_start..body_start + content_len].to_vec();
    Ok(HttpRequestParsed {
        method,
        target,
        headers,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_parser_round_trips_canonical_pull() {
        let raw =
            b"GET /v1/pull?max=4 HTTP/1.1\r\nHost: localhost\r\nX-QSL-Route-Token: valid_route_token_value_1234\r\n\r\n";
        let parsed = parse_http_request_bytes(raw).expect("request parses");
        assert_eq!(parsed.method, "GET");
        assert_eq!(
            parse_http_target(parsed.target.as_str()),
            Some(HttpRelayTarget::Pull(4))
        );
        assert_eq!(
            parse_http_route_token_from_request(&parsed).expect("route token"),
            "valid_route_token_value_1234"
        );
    }

    #[test]
    fn query_only_route_tokens_are_rejected() {
        let raw = b"GET /v1/pull?max=1&route_token=valid_route_token_value_1234 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let parsed = parse_http_request_bytes(raw).expect("request parses");
        assert_eq!(
            parse_http_route_token_from_request(&parsed).unwrap_err(),
            "missing_route_token"
        );
    }
}
