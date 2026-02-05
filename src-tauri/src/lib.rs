use std::sync::Arc;
use tauri::{Manager, WindowEvent, Emitter};
use tokio::sync::{Mutex, RwLock};

mod db;
mod websocket_server;
mod commands;
mod types;

pub use types::AppState;
use websocket_server::WebSocketServer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(debug_assertions)]
                {
                    let _ = window.open_devtools();
                }
            }

            // WebSocket 서버 초기화
            let app_handle = app.handle().clone();
            let db = Arc::new(Mutex::new(None));
            let ws_server = WebSocketServer::new(app_handle, db.clone());

            let app_state = AppState {
                db,
                ws_server: Arc::new(RwLock::new(Some(ws_server))),
            };

            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::init_db,
            commands::start_websocket_server,
            commands::list_clients,
            commands::get_connected_clients,
            commands::get_client_status,
            commands::list_client_statuses,
            commands::save_client_memo,
            commands::get_client_memo,
            commands::get_logs,
            commands::clear_old_logs,
            commands::send_client_command,
            commands::save_server_settings,
            commands::get_server_settings,
        ])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.emit("request-close", ());
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
