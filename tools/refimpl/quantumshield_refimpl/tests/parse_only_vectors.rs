use quantumshield_refimpl::{Envelope, ProtocolMessage};
use serde::Deserialize;

#[derive(Deserialize)]
struct Root {
    envelope: Option<Vec<Fixture>>,
    messaging: Option<Vec<Fixture>>,
}

#[derive(Deserialize)]
struct Fixture {
    name: String,
    wire_hex: String,
    expect_ok: bool,
}

fn hex_to_bytes(s: &str) -> Vec<u8> {
    let s = s.trim();
    hex::decode(s).expect("hex")
}

#[test]
fn parse_only_vectors() {
    let data = std::fs::read_to_string("vectors/parse_only.json").expect("vectors");
    let root: Root = serde_json::from_str(&data).expect("json");

    if let Some(v) = root.envelope {
        for f in v {
            let b = hex_to_bytes(&f.wire_hex);
            let ok = Envelope::decode(&b).is_ok();
            assert_eq!(ok, f.expect_ok, "envelope fixture {}", f.name);
        }
    }

    if let Some(v) = root.messaging {
        for f in v {
            let b = hex_to_bytes(&f.wire_hex);
            let ok = ProtocolMessage::decode(&b).is_ok();
            assert_eq!(ok, f.expect_ok, "messaging fixture {}", f.name);
        }
    }
}
