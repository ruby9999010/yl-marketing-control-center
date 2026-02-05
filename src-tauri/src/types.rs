use crate::db::Db;
use crate::websocket_server::WebSocketServer;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct AppState {
    pub db: Arc<Mutex<Option<Db>>>,
    pub ws_server: Arc<RwLock<Option<WebSocketServer>>>,
}
