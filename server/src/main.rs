use std::collections::HashMap;

use std::env;
use std::sync::Arc;
use std::time::Duration;

use db::initialize_db;
use sqlx::SqlitePool;
use tcp::{broadcast_new_data, handle_client, heartbeat_task, StateMessage};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use web::initialize_axum_server;
use web::spotify::spotify_polling_task;
use web::weather::weather_polling_task;

type Clients = Arc<RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>>>;

pub mod web;
pub mod db;
pub mod tcp;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db = match initialize_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let listener = TcpListener::bind("0.0.0.0:2699").await.unwrap();
    println!("Listening on port 2699");

    let (state_sender, state_receiver) = mpsc::channel::<StateMessage>(100);

    tokio::spawn(initialize_axum_server(db.clone()));
    tokio::spawn(broadcast_new_data(clients.clone(), state_receiver));
    tokio::spawn(heartbeat_task(state_sender.clone()));
    tokio::spawn(spotify_polling_task(db.clone(), state_sender.clone()));
    tokio::spawn(weather_polling_task(state_sender.clone()));

    loop {
        if let Ok((stream, addr)) = listener.accept().await {
            let peer_addr = addr.to_string();
            
            let (sender, receiver) = mpsc::channel(100);
            clients.write().await.insert(peer_addr.clone(), sender);

            let clients_clone = clients.clone();
            tokio::spawn(handle_client(stream, peer_addr, clients_clone, receiver));
        }
    }
}
