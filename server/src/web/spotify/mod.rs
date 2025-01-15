use std::time::Duration;

use reqwest::Client;
use sqlx::SqlitePool;
use tokio::{sync::mpsc, time::interval};

use crate::{db::{get_token_from_db, update_oauth2_access_and_refresh_tokens}, tcp::StateMessage, web::oauth2::exchange_code_for_tokens};

use super::oauth2::{refresh_access_token, ExchangeCodePayload, RefreshTokenPayload};

pub async fn fetch_current_playing_track(db: &SqlitePool) -> anyhow::Result<Option<String>> {
    let client = Client::new();
    let url = "https://api.spotify.com/v1/me/player/currently-playing";

    let oauth2_token = match get_token_from_db(db, "spotify".to_string()).await? {
        Some(token) => token,
        None => return Ok(None),
    };

    let access_token = oauth2_token.access_token.clone();

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let mut track_name: Option<String> = None;
    let mut artist_name: Option<String> = None;

    if response.status().is_success() {
        println!("Successfully fetched currently playing track");
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
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        println!("Failed to fetch currently playing track: {}", error_text);

        if status == 401 {
            let payload = ExchangeCodePayload {
                code: oauth2_token.code.clone(),
                redirect_uri: oauth2_token.redirect_uri.clone(),
                client_id: oauth2_token.client_id.clone(),
                client_secret: oauth2_token.client_secret.clone(),
                get_token_url: oauth2_token.get_token_url.clone(),
            };
            let new_token = exchange_code_for_tokens(payload).await?;
            let _ = update_oauth2_access_and_refresh_tokens(db, "spotify".to_string(), new_token.access_token, new_token.refresh_token, chrono::Utc::now().naive_utc() + chrono::Duration::seconds(new_token.expires_in as i64)).await?;
        }

        return Ok(None);
    }

    let formatted = match (track_name, artist_name) {
        (Some(track_name), Some(artist_name)) => Some(format!("{} - {}", artist_name, track_name)),
        _ => None,
    };

    Ok(formatted)
}

pub async fn spotify_polling_task(
    db: SqlitePool,
    sender: mpsc::Sender<StateMessage>,
) -> anyhow::Result<()> {
    let mut interval = interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;

        if let Ok(Some(oauth2_token)) = get_token_from_db(&db, "spotify".to_string()).await {
            if oauth2_token.access_token.is_empty() || oauth2_token.refresh_token.is_empty() {
                println!("We don't have an access token or refresh token for Spotify, so we need to exchange the code for tokens");
                let payload = ExchangeCodePayload {
                    code: oauth2_token.code.clone(),
                    redirect_uri: oauth2_token.redirect_uri.clone(),
                    client_id: oauth2_token.client_id.clone(),
                    client_secret: oauth2_token.client_secret.clone(),
                    get_token_url: oauth2_token.get_token_url.clone(),
                };
                match exchange_code_for_tokens(payload).await {
                    Ok(tokens) => {
                        let _ = update_oauth2_access_and_refresh_tokens(&db, "spotify".to_string(), tokens.access_token, tokens.refresh_token, chrono::Utc::now().naive_utc() + chrono::Duration::seconds(tokens.expires_in as i64)).await?;
                        println!("Successfully exchanged code for tokens");
                    },
                    Err(e) => {
                        println!("Failed to exchange code for tokens: {}", e);
                    }
                }
            }

            if chrono::Utc::now().naive_utc() >= oauth2_token.expires_at {
                println!("Access token has expired, refreshing token");
                let payload = RefreshTokenPayload {
                    client_id: oauth2_token.client_id.clone(),
                    client_secret: oauth2_token.client_secret.clone(),
                    refresh_token: oauth2_token.refresh_token.clone(),
                    get_token_url: oauth2_token.get_token_url.clone(),
                };
                if let Ok(refresh_response) = refresh_access_token(payload).await {
                    let _ = update_oauth2_access_and_refresh_tokens(&db, "spotify".to_string(), refresh_response.access_token, oauth2_token.refresh_token, chrono::Utc::now().naive_utc() + chrono::Duration::seconds(refresh_response.expires_in as i64)).await?;
                    println!("Successfully refreshed access token");
                }   
            }
            
            println!("Checking currently playing track");
            if let Ok(Some(track)) = fetch_current_playing_track(&db).await {
                println!("Currently playing: {}", track);
                let payload = format!("Currently playing: {}", track);
                sender.send(StateMessage::TrackData(payload)).await?;
            }
        }
        
    }
}