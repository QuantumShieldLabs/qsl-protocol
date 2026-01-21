use qsl_tui::demo::{format_meta_line, DemoResult, Mode, PaddingInfo, PartyRole, PrivacyMode};

#[test]
fn meta_line_contains_required_keys() {
    let result = DemoResult {
        plaintext: "hello".to_string(),
        padding: PaddingInfo {
            plain_len: 5,
            padded_len: 256,
            bucket: 256,
        },
        ciphertext_len: 300,
        privacy_mode: PrivacyMode::Padded,
    };

    let line = format_meta_line(
        PartyRole::Sender,
        Mode::Relay,
        true,
        PrivacyMode::Padded,
        &result,
    );

    assert!(line.contains("QSL_TUI_META"));
    assert!(line.contains("role=sender"));
    assert!(line.contains("mode=relay"));
    assert!(line.contains("proxy=on"));
    assert!(line.contains("privacy=padded"));
    assert!(line.contains("plaintext_len=5"));
    assert!(line.contains("ciphertext_len=300"));
    assert!(line.contains("padded_len=256"));
    assert!(line.contains("bucket=256"));
}
