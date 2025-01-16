use core::send_message;
use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{mpsc, RwLock},
    time::interval,
};

pub type Clients = Arc<RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>>>;

pub enum StateMessage {
    TrackData(String),
    WeatherData(String),
    XtbData(String),
    Ping,
}

pub async fn handle_client(
    stream: TcpStream,
    peer_addr: String,
    clients: Clients,
    mut receiver: mpsc::Receiver<Vec<u8>>,
) {
    let (mut reader, mut writer) = stream.into_split();

    let peer_addr_clone = peer_addr.clone();
    tokio::spawn(async move {
        let mut buffer = [0; 1024];

        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    println!("Client {} disconnected", peer_addr_clone);
                    break;
                }
                Ok(size) => {
                    let received = String::from_utf8_lossy(&buffer[..size]);
                    println!("Received from {}: {}", peer_addr_clone, received);
                }
                Err(e) => {
                    println!("Error reading from client {}: {}", peer_addr_clone, e);
                    break;
                }
            }
        }
    });

    let peer_addr = peer_addr.clone();
    while let Some(message) = receiver.recv().await {
        if let Err(e) = writer.write_all(&message).await {
            println!("Error writing to client {}: {}", peer_addr.clone(), e);
            break;
        }
    }

    clients.write().await.remove(&peer_addr);
    println!("Client {} disconnected", peer_addr);
}

pub async fn broadcast_new_data(
    clients: Clients,
    mut state_receiver: mpsc::Receiver<StateMessage>,
) {
    async fn broadcast_to_clients(clients: &Clients, payload: Vec<u8>, data_type: &str) {
        let clients_lock = clients.read().await;
        for (_, sender) in clients_lock.iter() {
            if sender.send(payload.clone()).await.is_err() {
                println!("Client disconnected");
            }
        }

        println!(
            "Broadcasted {}!",
            data_type,
        );
    }

    while let Some(state) = state_receiver.recv().await {
        let (data_type, payload) = match state {
            StateMessage::TrackData(track_data) => ("Spotify", send_message("Spotify", &track_data)),
            StateMessage::WeatherData(weather_data) => ("Weather", send_message("Weather", &weather_data)),
            StateMessage::XtbData(xtb_data) => ("XTB", send_message("XTB", &xtb_data)),
            _ => {
                println!("Unknown message");
                continue;
            }
        };

        match payload {
            Ok(payload) => {
                broadcast_to_clients(&clients, payload, data_type).await;
            }
            Err(e) => {
                eprintln!("Failed to send message for {}: {:?}", data_type, e);
            }
        }
    }
}

pub async fn heartbeat_task(sender: mpsc::Sender<StateMessage>) -> anyhow::Result<()> {
    let mut interval = interval(Duration::from_secs(10));

    loop {
        interval.tick().await;

        sender.send(StateMessage::Ping).await?;
    }
}
