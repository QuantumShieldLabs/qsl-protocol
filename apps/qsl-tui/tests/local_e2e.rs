use qsl_tui::demo::{run_demo, Mode};

#[tokio::test]
async fn local_demo_encrypts_and_decrypts() {
    let out = run_demo(Mode::Local, "http://127.0.0.1:8080", "demo")
        .await
        .expect("local demo should succeed");
    assert_eq!(out.plaintext, "hello");
}
