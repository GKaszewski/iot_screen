use core::send_message;
use std::collections::HashMap;

use std::env;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;
use sqlx::Row;
use sqlx::{Sqlite, SqlitePool};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;

type Clients = Arc<RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>>>;

async fn handle_client(stream: TcpStream,
    peer_addr: String,
    clients: Clients,
    mut receiver: mpsc::Receiver<Vec<u8>>
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

const DB_URL: &str = "sqlite://db.sqlite";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
    #[serde(rename = "utc_offset_seconds")]
    pub utc_offset_seconds: i64,
    pub timezone: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(rename = "current_units")]
    pub current_units: CurrentUnits,
    pub current: Current,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUnits {
    pub time: String,
    pub interval: String,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub time: String,
    pub interval: i64,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: f64,
}

async fn get_weather_data() -> anyhow::Result<WeatherResponse> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=54.3523&longitude=18.6491&current=temperature_2m&timezone=Europe%2FBerlin";
    let response = reqwest::get(url).await?;
    let data = response.json::<WeatherResponse>().await?;
    Ok(data)
}

async fn get_latest_weather_from_db(pool: &SqlitePool) -> anyhow::Result<Option<f64>> {
    let query = r#"
        SELECT temperature
        FROM weather
        ORDER BY time DESC
        LIMIT 1
    "#;

    let row = sqlx::query(query).fetch_one(pool).await.ok();

    match row {
        Some(row) => Ok(Some(row.get(0))),
        None => Ok(None),
    }
}

async fn save_weather_to_database(
    pool: &SqlitePool,
    weather_data: WeatherResponse,
) -> anyhow::Result<()> {
    let query = r#"
        INSERT INTO weather (time, interval, temperature)
        VALUES (?, ?, ?)
    "#;

    sqlx::query(query)
        .bind(weather_data.current.time)
        .bind(weather_data.current.interval)
        .bind(weather_data.current.temperature_2m)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Database created"),
            Err(e) => println!("Error creating database: {}", e),
        }
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    match sqlx::migrate!("./migrations").run(&db).await {
        Ok(_) => println!("Migrations applied"),
        Err(e) => println!("Error applying migrations: {}", e),
    }

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let listener = TcpListener::bind("0.0.0.0:2699").await.unwrap();
    
    println!("Listening on port 2699");

    let weather = get_weather_data().await.unwrap();
    save_weather_to_database(&db, weather).await.unwrap();

    let spotify_client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
    let spotify_client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");
    let code = env::var("SPOTIFY_CODE").expect("SPOTIFY_CODE not set");

    let track_data: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));

    if let Ok((access_token, refresh_token, expires_in)) = exchange_code_for_tokens(
        &spotify_client_id,
        &spotify_client_secret,
        &code,
        "https://localhost:2700",
    ).await {
        save_tokens(
            &db,
            &access_token,
            &refresh_token,
            chrono::Utc::now().naive_utc() + chrono::Duration::seconds(expires_in as i64),
        )
        .await
        .unwrap();
    } else {
        println!("Failed to exchange code for tokens");
    }


    tokio::spawn(broadcast_new_data(clients.clone(), db.clone(), track_data.clone()));
    tokio::spawn(heartbeat_task(clients.clone()));
    tokio::spawn(spotify_polling_task(db.clone(), spotify_client_id, Some(spotify_client_secret.to_string()), track_data.clone()));

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

async fn broadcast_new_data(clients: Clients, db: SqlitePool, track_data: Arc<RwLock<Option<String>>>) {
    loop {
        let current_client_count = clients.read().await.len();
        println!("Current client count: {}", current_client_count);

        let weather_message = if let Ok(Some(current_temperature)) = get_latest_weather_from_db(&db).await {
            Some(format!("Current temperature: {}", current_temperature))
        } else {
            None
        };

        let track_message = {
            let track_data_lock = track_data.read().await;
            track_data_lock.clone()
        };

        let clients_lock = clients.read().await;

        if let Some(weather) = weather_message {
            let weather_payload = send_message("WEATHER", &weather).unwrap();
            for (_, sender) in clients_lock.iter() {
                if sender.send(weather_payload.clone()).await.is_err() {
                    println!("Client disconnected");
                }
            }

            println!("Broadcasted weather data");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        if let Some(track) = track_message {
            let track_payload = send_message("SPOTIFY", &track).unwrap();
            for (_, sender) in clients_lock.iter() {
                if sender.send(track_payload.clone()).await.is_err() {
                    println!("Client disconnected");
                }
            }

            println!("Broadcasted track data");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn heartbeat_task(clients: Clients) {
    let mut interval = interval(Duration::from_secs(10));

    loop {
        interval.tick().await;

        let message = "HEARTBEAT";
        let payload = send_message("PING", &message).unwrap();

        let clients_lock = clients.read().await;
        for (peer_addr, sender) in clients_lock.iter() {
            if sender.send(payload.clone()).await.is_err() {
                println!("Client {} disconnected", peer_addr);
            }
        }
    }
}

async fn get_api_key(pool: &SqlitePool, service_name: &str) -> anyhow::Result<Option<String>> {
    let query = r#"
        SELECT api_key
        FROM api_keys
        WHERE service_name = ?
    "#;

    let row = sqlx::query(query).bind
    (service_name).fetch_one(pool).await.ok();

    match row {
        Some(row) => Ok(Some(row.get(0))),
        None => Ok(None),
    }
}

async fn add_new_api_key(pool: &SqlitePool, service_name: &str, api_key: &str) -> anyhow::Result<()> {
    let query = r#"
        INSERT INTO api_keys (service_name, api_key)
        VALUES (?, ?)
    "#;

    sqlx::query(query)
        .bind(service_name)
        .bind(api_key)
        .execute(pool)
        .await?;

    Ok(())
}

async fn delete_api_key(pool: &SqlitePool, service_name: &str) -> anyhow::Result<()> {
    let query = r#"
        DELETE FROM api_keys
        WHERE service_name = ?
    "#;

    sqlx::query(query)
        .bind(service_name)
        .execute(pool)
        .await?;

    Ok(())
}

async fn fetch_current_playing_track(access_token: &str) -> anyhow::Result<Option<String>> {
    let client = Client::new();
    let url = "https://api.spotify.com/v1/me/player/currently-playing";

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let mut track_name: Option<String> = None;
    let mut artist_name: Option<String> = None;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        if let Some(item) = json.get("item") {
            if let Some(track) = item.get("name") {
                track_name = Some(track.to_string());
            }
            if let Some(artists) = item.get("artists") {
                if let Some(artist) = artists.get(0) {
                    if let Some(artist) = artist.get("name") {
                        artist_name = Some(artist.to_string());
                    }
                }
            }
        } 
    }

    let formatted = match (track_name, artist_name) {
        (Some(track_name), Some(artist_name)) => Some(format!("{} - {}", artist_name, track_name)),
        _ => None,
    };

    Ok(formatted)
}

async fn spotify_polling_task(db: SqlitePool, client_id: String, client_secret: Option<String>, track_data: Arc<RwLock<Option<String>>>) {
    let mut interval = interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;

        if let Ok(Some((access_token, refresh_token, expires_at))) = get_token_from_db(&db).await
        {
            if chrono::Utc::now().naive_utc() >= expires_at {
                // Refresh token
                if let Ok((new_access_token, expires_in)) =
                    refresh_access_token(&client_id.clone(), client_secret.clone(), &refresh_token).await
                {
                    save_tokens(
                        &db,
                        &new_access_token,
                        &refresh_token,
                        chrono::Utc::now()
                            .naive_utc()
                            + chrono::Duration::seconds(expires_in as i64),
                    )
                    .await
                    .unwrap();
                }
            }

            if let Ok(Some(track)) = fetch_current_playing_track(&access_token).await {
                println!("Currently playing: {}", track);
                let mut track_data_lock = track_data.write().await;
                *track_data_lock = Some(track);
            }
        } else {
            println!("No tokens found in database");
        }
    }
}

pub async fn exchange_code_for_tokens(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> anyhow::Result<(String, String, u64)> {
    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let access_token = json.get("access_token").unwrap().as_str().unwrap();
        let refresh_token = json.get("refresh_token").unwrap().as_str().unwrap();
        let expires_in = json.get("expires_in").unwrap().as_u64().unwrap();

        Ok((access_token.to_string(), refresh_token.to_string(), expires_in))
    } else {
        let error_text = response.text().await?;
        println!("Failed to exchange code for tokens: {}", error_text);
        Err(anyhow::anyhow!("Failed to exchange code for tokens: {}", error_text))
    }
}

async fn save_tokens(
    pool: &SqlitePool,
    access_token: &str,
    refresh_token: &str,
    expires_at: chrono::NaiveDateTime,
) -> anyhow::Result<()> {
    let query = r#"
        INSERT INTO spotify_tokens (access_token, refresh_token, expires_at)
        VALUES (?, ?, ?)
    "#;

    sqlx::query(query)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .execute(pool)
        .await?;

    Ok(())
}

async fn refresh_access_token(
    client_id: &str,
    client_secret: Option<String>,
    refresh_token: &str,
) -> anyhow::Result<(String, u64)> {
    let client = Client::new();
    let mut params = Vec::new();
    if let Some(client_secret) = client_secret {
        params.push(("client_secret", client_secret));
    } else {
        params.push(("client_id", client_id.to_string()));
        params.push(("grant_type", "refresh_token".to_string()));
        params.push(("refresh_token", refresh_token.to_string()));
    }

   
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let access_token = json.get("access_token").unwrap().as_str().unwrap();
        let expires_in = json.get("expires_in").unwrap().as_u64().unwrap();

        Ok((access_token.to_string(), expires_in))
    } else {
        Err(anyhow::anyhow!("Failed to refresh access token"))
    }
}

async fn get_token_from_db(pool: &SqlitePool) -> anyhow::Result<Option<(String, String, chrono::NaiveDateTime)>> {
    let query = r#"
        SELECT access_token, refresh_token, expires_at
        FROM spotify_tokens
        ORDER BY id DESC
        LIMIT 1
    "#;

    let row = sqlx::query(query).fetch_one(pool).await.ok();

    match row {
        Some(row) => {
            let access_token = row.get(0);
            let refresh_token = row.get(1);
            let expires_at = row.get(2);

            Ok(Some((access_token, refresh_token, expires_at)))
        }
        None => Ok(None),
    }
}