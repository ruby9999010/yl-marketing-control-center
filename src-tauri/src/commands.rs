use crate::db::{Client, ClientMemo, ClientStatus, Db, LogEntry};
use crate::types::AppState;
use crate::websocket_server::WebSocketServer;
use anyhow::Result;
use tauri::State;

// 데이터베이스 초기화
#[tauri::command]
pub async fn init_db(app_state: State<'_, AppState>) -> Result<(), String> {
    let db = Db::new().await.map_err(|e| e.to_string())?;
    *app_state.db.lock().await = Some(db);
    Ok(())
}

// WebSocket 서버 시작
#[tauri::command]
pub async fn start_websocket_server(
    app_state: State<'_, AppState>,
    port: u16,
) -> Result<(), String> {
    let ws_server = app_state.ws_server.read().await;
    
    if let Some(server) = ws_server.as_ref() {
        server.start(port).await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("WebSocket server not initialized".to_string())
    }
}

// 클라이언트 목록 조회
#[tauri::command]
pub async fn list_clients(app_state: State<'_, AppState>) -> Result<Vec<Client>, String> {
    let db = app_state.db.lock().await;
    let db = db.as_ref().ok_or("Database not initialized")?;
    
    db.list_clients().await.map_err(|e| e.to_string())
}

// 연결된 클라이언트 목록
#[tauri::command]
pub async fn get_connected_clients(app_state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let ws_server = app_state.ws_server.read().await;
    
    if let Some(server) = ws_server.as_ref() {
        Ok(server.get_connected_clients().await)
    } else {
        Ok(vec![])
    }
}

// 클라이언트 상태 조회
#[tauri::command]
pub async fn get_client_status(
    app_state: State<'_, AppState>,
    client_id: String,
) -> Result<Option<ClientStatus>, String> {
    let db = app_state.db.lock().await;
    let db = db.as_ref().ok_or("Database not initialized")?;
    
    db.get_status(&client_id).await.map_err(|e| e.to_string())
}

// 모든 클라이언트 상태 조회
#[tauri::command]
pub async fn list_client_statuses(app_state: State<'_, AppState>) -> Result<Vec<ClientStatus>, String> {
    let db = app_state.db.lock().await;
    let db = db.as_ref().ok_or("Database not initialized")?;
    
    db.list_statuses().await.map_err(|e| e.to_string())
}

// 메모 저장
#[tauri::command]
pub async fn save_client_memo(
    app_state: State<'_, AppState>,
    client_id: String,
    memo: String,
) -> Result<(), String> {
    let db = app_state.db.lock().await;
    let db = db.as_ref().ok_or("Database not initialized")?;
    
    db.save_memo(&client_id, &memo).await.map_err(|e| e.to_string())
}

// 메모 조회
#[tauri::command]
pub async fn get_client_memo(
    app_state: State<'_, AppState>,
    client_id: String,
) -> Result<Option<ClientMemo>, String> {
    let db = app_state.db.lock().await;
    let db = db.as_ref().ok_or("Database not initialized")?;
    
    db.get_memo(&client_id).await.map_err(|e| e.to_string())
}

// 로그 조회
#[tauri::command]
pub async fn get_logs(
    app_state: State<'_, AppState>,
    client_id: Option<String>,
    limit: i64,
) -> Result<Vec<LogEntry>, String> {
    let db = app_state.db.lock().await;
    let db = db.as_ref().ok_or("Database not initialized")?;
    
    db.get_logs(client_id.as_deref(), limit).await.map_err(|e| e.to_string())
}

// 오래된 로그 삭제
#[tauri::command]
pub async fn clear_old_logs(
    app_state: State<'_, AppState>,
    days: i64,
) -> Result<u64, String> {
    let db = app_state.db.lock().await;
    let db = db.as_ref().ok_or("Database not initialized")?;
    
    db.clear_old_logs(days).await.map_err(|e| e.to_string())
}

// 클라이언트에 명령 전송
#[tauri::command]
pub async fn send_client_command(
    app_state: State<'_, AppState>,
    client_id: String,
    command: String,
) -> Result<(), String> {
    let ws_server = app_state.ws_server.read().await;
    
    if let Some(server) = ws_server.as_ref() {
        server.send_command(&client_id, &command).await.map_err(|e| e.to_string())
    } else {
        Err("WebSocket server not initialized".to_string())
    }
}

// 서버 설정 저장
#[tauri::command]
pub async fn save_server_settings(
    app_state: State<'_, AppState>,
    port: u16,
) -> Result<(), String> {
    // 설정을 파일이나 DB에 저장 (향후 구현)
    println!("Server port saved: {}", port);
    Ok(())
}

// 서버 설정 조회
#[tauri::command]
pub async fn get_server_settings(
    app_state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "port": 9999
    }))
}
