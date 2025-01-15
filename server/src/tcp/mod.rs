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
    let mut interval = interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            Some(state) = state_receiver.recv() => {
                match state {
                    StateMessage::TrackData(track_data) => {
                        let payload = send_message("Spotify", &track_data).unwrap();
                        let clients_lock = clients.read().await;
                        for (_, sender) in clients_lock.iter() {
                            if sender.send(payload.clone()).await.is_err() {
                                println!("Client disconnected");
                            }
                            println!("Broadcasted track data: {}", track_data);
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }


                    },
                    StateMessage::WeatherData(weather_data) => {
                        let payload = send_message("Weather", &weather_data).unwrap();
                        let clients_lock = clients.read().await;
                        for (_, sender) in clients_lock.iter() {
                            if sender.send(payload.clone()).await.is_err() {
                                println!("Client disconnected");
                            }
                            println!("Broadcasted weather data: {}", weather_data);
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }


                    },
                    StateMessage::Ping => {
                        let payload = send_message("Ping", "PONG").unwrap();
                        let clients_lock = clients.read().await;
                        for (_, sender) in clients_lock.iter() {
                            if sender.send(payload.clone()).await.is_err() {
                                println!("Client disconnected");
                            }
                            println!("Broadcasted PING");
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }


                    }
                }
            },
            _ = interval.tick() => {
               let current_client_count = clients.read().await.len();
                println!("Current client count: {}", current_client_count);
            },
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
