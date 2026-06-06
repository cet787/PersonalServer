use axum::{
    routing::get,
    extract::{Path, State},
    Json,
    Router,
};
use std::collections::HashMap;
use serde_json::json;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use serde::Serialize;

use PersonalServer::live_server::start_tcp_server;

type SharedState = Arc<Mutex<AppState>>;

#[derive(Serialize)]
enum Sensor {
    Int(SensorData<i32>),
    Float(SensorData<f64>),
    Bool(SensorData<bool>),
}

struct AppState {
    status: String,
    health: u64,
    sensors: HashMap<String, Sensor>
}

impl AppState {
    fn get_json(&self) -> Json<serde_json::Value> {
        Json(json!({
            "status": self.status,
            "health": self.health,
            "sensors": self.sensors
        }))
    }
    fn add_sensor(&mut self, name:String, sensor: Sensor) {
        self.sensors.insert(name, sensor);
    }
}

#[derive(Serialize)]
struct SetResponse<T> {
    message: String,
    value: T,
}

#[derive(Serialize)]
struct SensorData<T>
where
    T: PartialOrd + Serialize,
{
    name: String,
    description: String,
    value: T,
    limit: T,
    warning: bool,
}

impl<T: PartialOrd + Serialize> SensorData<T> {
    fn get_json(&self) -> Json<serde_json::Value> {
        Json(json!({
            "name": self.name,
            "description": self.description,
            "value": self.value,
            "limit": self.limit,
            "warning": self.warning,
        }))
    }
}

async fn health(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let state = state.lock().unwrap();
    Json(json!({
        "status": state.status,
        "health": state.health,
    }))
}

async fn set_health(State(state): State<SharedState>, Path(value): Path<u64>) -> Json<SetResponse<u64>> {
    let mut state = state.lock().unwrap();
    state.health = value;
    Json(SetResponse {
        message: format!("Health value successfully set to: {}", state.health).to_string(),
        value: state.health,
    })
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState {
        status: "ok".into(),
        health: 0,
        sensors: HashMap::new(),
    }));

    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap();

    let app = Router::new()
        .route("/health", get(health))
        .route("/set_health/{value}", get(set_health))
        .with_state(state);

    let listener =
        TcpListener::bind(("0.0.0.0", port))
            .await
            .unwrap();


    tokio::spawn(async move { start_tcp_server().await.unwrap() });

    println!("Local TCP server started on localhost:{}", 8080);

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}