#![no_main]

use libfuzzer_sys::fuzz_target;
use qsc::adversarial::payload::{
    parse_attachment_confirm_payload, parse_attachment_descriptor_payload,
    parse_file_confirm_payload, parse_file_transfer_payload, parse_receipt_payload,
};

fuzz_target!(|data: &[u8]| {
    let _ = parse_receipt_payload(data);
    let _ = parse_file_confirm_payload(data);
    let _ = parse_file_transfer_payload(data);
    let _ = parse_attachment_descriptor_payload(data);
    let _ = parse_attachment_confirm_payload(data);
});
