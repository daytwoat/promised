use anyhow::Result;
use axum::{Json, Router, extract::State, routing::post};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::blocklist::Blocklist;

const PORT: u16 = 8080;

pub async fn run_http_server(state: Arc<RwLock<Blocklist>>) -> Result<()> {
    let app = Router::new()
        .route("/config", post(save_config).get(get_config))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    println!("HTTP API running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn get_config1() -> Json<Blocklist> {
    match std::fs::read_to_string("config.json") {
        Ok(contents) if contents.trim().is_empty() => Json(Blocklist { domains: vec![] }),
        Ok(contents) => match serde_json::from_str::<Blocklist>(&contents) {
            Ok(config) => Json(config),
            Err(_) => Json(Blocklist { domains: vec![] }),
        },
        Err(_) => Json(Blocklist { domains: vec![] }),
    }
}

async fn get_config(State(state): State<Arc<RwLock<Blocklist>>>) -> Json<Blocklist> {
    let config = state.read().await;
    Json(config.clone())
}

async fn save_config(
    State(state): State<Arc<RwLock<Blocklist>>>,
    Json(payload): Json<Blocklist>,
) -> Json<&'static str> {
    //global meomory update
    {
        let mut config = state.write().await;
        *config = payload.clone();
    }

    if let Ok(json) = serde_json::to_string_pretty(&payload) {
        let _ = std::fs::write("config.json", json);
    }

    Json("saved")
}
