#![no_main]

use libfuzzer_sys::fuzz_target;
use qsc::adversarial::vault_format::parse_vault_envelope;

fuzz_target!(|data: &[u8]| {
    let _ = parse_vault_envelope(data);
});
