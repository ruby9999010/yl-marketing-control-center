use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::db::Db;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientMessage {
    #[serde(rename = "register")]
    Register {
        client_id: String,
        timestamp: i64,
        version: String,
    },
    #[serde(rename = "status")]
    Status {
        client_id: String,
        data: StatusData,
    },
    #[serde(rename = "log")]
    Log {
        client_id: String,
        level: String,
        message: String,
        timestamp: i64,
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "heartbeat")]
    Heartbeat {
        client_id: String,
        timestamp: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusData {
    pub success_count: i64,
    pub failure_count: i64,
    pub next_execution_in: i64,
    pub current_status: String,
    pub last_activity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    #[serde(rename = "registered")]
    Registered {
        client_id: String,
        status: String,
        message: String,
    },
    #[serde(rename = "command")]
    Command {
        command: String,
        client_id: String,
    },
    #[serde(rename = "ping")]
    Ping {
        timestamp: i64,
    },
}

type ClientSender = mpsc::UnboundedSender<Message>;

pub struct WebSocketServer {
    clients: Arc<RwLock<HashMap<String, ClientSender>>>,
    db: Arc<RwLock<Option<Db>>>,
    app_handle: AppHandle,
}

impl WebSocketServer {
    pub fn new(app_handle: AppHandle, db: Arc<RwLock<Option<Db>>>) -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            db,
            app_handle,
        }
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        // 로컬호스트만 허용하려면 127.0.0.1 사용
        // 네트워크 접근 허용하려면 0.0.0.0 사용
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
        let listener = TcpListener::bind(&addr).await?;
        
        println!("🚀 WebSocket server listening on {}", addr);
        println!("💡 네트워크 접근 허용됨 - 클라이언트가 연결할 수 있습니다");
        let _ = self.app_handle.emit("ws-server-started", serde_json::json!({ "port": port }));

        let clients = self.clients.clone();
        let db = self.db.clone();
        let app_handle = self.app_handle.clone();

        tokio::spawn(async move {
            while let Ok((stream, peer_addr)) = listener.accept().await {
                println!("📥 New connection from: {}", peer_addr);
                
                let clients_clone = clients.clone();
                let db_clone = db.clone();
                let app_handle_clone = app_handle.clone();

                tokio::spawn(async move {
                    if let Err(e) = Self::handle_connection(
                        stream,
                        peer_addr,
                        clients_clone,
                        db_clone,
                        app_handle_clone,
                    ).await {
                        eprintln!("❌ Connection error: {}", e);
                    }
                });
            }
        });

        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        peer_addr: SocketAddr,
        clients: Arc<RwLock<HashMap<String, ClientSender>>>,
        db: Arc<RwLock<Option<Db>>>,
        app_handle: AppHandle,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
        let mut client_id: Option<String> = None;

        // 송신 태스크
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if ws_sender.send(msg).await.is_err() {
                    break;
                }
            }
        });

        // 수신 루프
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        match client_msg {
                            ClientMessage::Register { client_id: cid, version, .. } => {
                                println!("✅ Client registered: {} (v{})", cid, version);
                                
                                // DB에 등록
                                if let Some(db) = db.read().await.as_ref() {
                                    let _ = db.register_client(&cid, None).await;
                                }

                                // 클라이언트 맵에 추가
                                clients.write().await.insert(cid.clone(), tx.clone());
                                client_id = Some(cid.clone());

                                // 등록 확인 메시지 전송
                                let response = ServerMessage::Registered {
                                    client_id: cid.clone(),
                                    status: "approved".to_string(),
                                    message: "연결되었습니다".to_string(),
                                };

                                if let Ok(json) = serde_json::to_string(&response) {
                                    let _ = tx.send(Message::Text(json));
                                }

                                // UI에 알림
                                let _ = app_handle.emit("client-connected", serde_json::json!({
                                    "clientId": cid,
                                    "version": version
                                }));
                            }
                            ClientMessage::Status { client_id: cid, data } => {
                                println!("📊 Status from {}: {:?}", cid, data);
                                
                                // DB에 상태 저장
                                if let Some(db) = db.read().await.as_ref() {
                                    let _ = db.update_status(
                                        &cid,
                                        data.success_count,
                                        data.failure_count,
                                        data.next_execution_in,
                                        &data.current_status,
                                        Some(&data.last_activity),
                                    ).await;
                                    let _ = db.update_client_last_seen(&cid).await;
                                }

                                // UI에 상태 업데이트
                                let _ = app_handle.emit("client-status-update", serde_json::json!({
                                    "clientId": cid,
                                    "data": data
                                }));
                            }
                            ClientMessage::Log { client_id: cid, level, message, metadata, .. } => {
                                println!("[{}] {}: {}", cid, level, message);
                                
                                // DB에 로그 저장
                                if let Some(db) = db.read().await.as_ref() {
                                    let metadata_str = metadata.as_ref()
                                        .and_then(|m| serde_json::to_string(m).ok());
                                    let _ = db.add_log(&cid, &level, &message, metadata_str.as_deref()).await;
                                    let _ = db.update_client_last_seen(&cid).await;
                                }

                                // UI에 로그 전송
                                let _ = app_handle.emit("client-log", serde_json::json!({
                                    "clientId": cid,
                                    "level": level,
                                    "message": message,
                                    "metadata": metadata
                                }));
                            }
                            ClientMessage::Heartbeat { client_id: cid, .. } => {
                                // 하트비트 처리
                                if let Some(db) = db.read().await.as_ref() {
                                    let _ = db.update_client_last_seen(&cid).await;
                                }
                            }
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    let _ = tx.send(Message::Pong(data));
                }
                Ok(Message::Close(_)) => {
                    println!("👋 Client disconnected: {:?}", client_id);
                    break;
                }
                Err(e) => {
                    eprintln!("❌ WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        // 연결 종료 처리
        if let Some(cid) = client_id {
            clients.write().await.remove(&cid);
            
            if let Some(db) = db.read().await.as_ref() {
                let _ = db.deactivate_client(&cid).await;
            }

            let _ = app_handle.emit("client-disconnected", serde_json::json!({
                "clientId": cid
            }));
        }

        println!("🔌 Connection closed: {}", peer_addr);
        Ok(())
    }

    pub async fn send_command(&self, client_id: &str, command: &str) -> Result<()> {
        let clients = self.clients.read().await;
        
        if let Some(sender) = clients.get(client_id) {
            let msg = ServerMessage::Command {
                command: command.to_string(),
                client_id: client_id.to_string(),
            };

            let json = serde_json::to_string(&msg)?;
            sender.send(Message::Text(json))?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Client not connected"))
        }
    }

    pub async fn get_connected_clients(&self) -> Vec<String> {
        self.clients.read().await.keys().cloned().collect()
    }
}
