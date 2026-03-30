#![no_main]

use libfuzzer_sys::fuzz_target;
use qsc::adversarial::route::{
    parse_http_request_bytes, parse_http_route_token_from_request, parse_http_target,
};

fuzz_target!(|data: &[u8]| {
    if let Ok(req) = parse_http_request_bytes(data) {
        let _ = parse_http_target(req.target.as_str());
        let _ = parse_http_route_token_from_request(&req);
    }
});
