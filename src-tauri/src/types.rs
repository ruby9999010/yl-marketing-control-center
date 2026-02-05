use crate::db::Db;
use crate::websocket_server::WebSocketServer;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Arc<RwLock<Option<Db>>>,
    pub ws_server: Arc<RwLock<Option<WebSocketServer>>>,
}
