use axum::{extract::{State, Json}, response::IntoResponse, routing::{get, post}, Router};
use serde::Deserialize;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::{cors::{AllowHeaders, CorsLayer}, services::ServeDir};

use crate::db::{add_new_oauth2_token_to_db, OAuth2Token};

pub mod weather;
pub mod oauth2;
pub mod spotify;

pub async fn initialize_axum_server(
    db: SqlitePool,
) -> anyhow::Result<()> {
    let origins = [
        "http://localhost:5173".parse().unwrap(),
        "http://localhost:8080".parse().unwrap(),
        "http://localhost:2700".parse().unwrap(),
    ];

    let app = Router::new()
    .route("/health", get(health_check))
    .route("/oauth2/code", post(post_oauth2_code))
    .layer(
        CorsLayer::new()
        .allow_origin(origins)
        .allow_headers(AllowHeaders::any())
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
    )
    .fallback_service(ServeDir::new("frontend/dist"))
    .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:2700").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    "OK"
}

#[derive(Deserialize)]
struct PostOAuth2Payload {
    code: String,
    #[serde(rename = "appName")]
    app_name: String,
    #[serde(rename = "clientSecret")]
    client_secret: String,
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "redirectUri")]
    redirect_uri: String,
    #[serde(rename = "getTokenUrl")]
    get_token_url: String,
}

async fn post_oauth2_code(State(db): State<SqlitePool>, Json(payload): Json<PostOAuth2Payload>) -> impl IntoResponse {
    println!("Hello from post_oauth2_code");
    let PostOAuth2Payload { code, app_name, client_secret, client_id, redirect_uri, get_token_url } = payload;
    println!("Received OAuth2 code for app: {}", app_name);

    match app_name.as_str() {
        "spotify" => {
            let data = OAuth2Token {
                app_name: app_name,
                client_secret: client_secret,
                client_id: client_id,
                redirect_uri: redirect_uri,
                access_token: "".to_string(),
                refresh_token: "".to_string(),
                expires_at: chrono::Utc::now().naive_utc(),
                code: code,
                get_token_url: get_token_url,
                created_at: chrono::Utc::now().naive_utc(),
            };
            let _ = add_new_oauth2_token_to_db(
                &db,
                data,
            ).await;
        },
        _ => return "Invalid app name".into_response(),
    }

    axum::http::StatusCode::OK.into_response()
}