use qsl_tui::demo::{run_demo, Mode, PrivacyMode};

#[tokio::test]
async fn padded_mode_reports_bucketed_ciphertext() {
    let out = run_demo(
        Mode::Local,
        "http://127.0.0.1:8080",
        "demo",
        PrivacyMode::Padded,
    )
    .await
    .expect("padded demo should succeed");

    assert_eq!(out.plaintext, "hello");
    assert!(out.ciphertext_len >= out.padding.plain_len);
    assert!(out.padding.padded_len >= out.padding.plain_len);
    assert!(out.padding.bucket >= out.padding.padded_len);
    assert!([256usize, 512, 1024, 2048, 4096, 8192].contains(&out.padding.bucket));
}
