mod model;
mod qsc;

use crate::model::{AppSnapshot, ReceiveResult, SendResult, UiError};
use crate::qsc::DesktopRuntime;
use tauri::AppHandle;

#[tauri::command]
fn refresh_snapshot(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    selected_peer: Option<String>,
) -> Result<AppSnapshot, UiError> {
    runtime.refresh_snapshot(&app, selected_peer)
}

#[tauri::command]
fn init_passphrase_profile(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    passphrase: String,
    selected_peer: Option<String>,
) -> Result<AppSnapshot, UiError> {
    runtime.init_passphrase_profile(&app, passphrase, selected_peer)
}

#[tauri::command]
fn unlock_profile(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    passphrase: String,
    selected_peer: Option<String>,
) -> Result<AppSnapshot, UiError> {
    runtime.unlock_profile(&app, passphrase, selected_peer)
}

#[tauri::command]
fn clear_session_unlock(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    selected_peer: Option<String>,
) -> Result<AppSnapshot, UiError> {
    runtime.clear_session_unlock(&app, selected_peer)
}

#[tauri::command]
fn set_inbox_token(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    token: String,
    selected_peer: Option<String>,
) -> Result<AppSnapshot, UiError> {
    runtime.set_inbox_token(&app, token, selected_peer)
}

#[tauri::command]
fn add_contact(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    label: String,
    fingerprint: String,
    route_token: Option<String>,
    selected_peer: Option<String>,
) -> Result<AppSnapshot, UiError> {
    runtime.add_contact(&app, label, fingerprint, route_token, selected_peer)
}

#[tauri::command]
fn trust_device(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    label: String,
    device_id: String,
    selected_peer: Option<String>,
) -> Result<AppSnapshot, UiError> {
    runtime.trust_device(&app, label, device_id, selected_peer)
}

#[tauri::command]
fn send_message(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    relay_url: String,
    label: String,
    message: String,
    selected_peer: Option<String>,
) -> Result<SendResult, UiError> {
    runtime.send_message(&app, relay_url, label, message, selected_peer)
}

#[tauri::command]
fn receive_messages(
    app: AppHandle,
    runtime: tauri::State<'_, DesktopRuntime>,
    relay_url: String,
    label: String,
    max_items: usize,
    selected_peer: Option<String>,
) -> Result<ReceiveResult, UiError> {
    runtime.receive_messages(&app, relay_url, label, max_items, selected_peer)
}

fn main() {
    tauri::Builder::default()
        .manage(DesktopRuntime::default())
        .invoke_handler(tauri::generate_handler![
            refresh_snapshot,
            init_passphrase_profile,
            unlock_profile,
            clear_session_unlock,
            set_inbox_token,
            add_contact,
            trust_device,
            send_message,
            receive_messages,
        ])
        .run(tauri::generate_context!())
        .expect("tauri app run");
}
