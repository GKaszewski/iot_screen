use crate::web::weather::WeatherResponse;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[allow(dead_code, unused)]

const DB_URL: &str = "sqlite://db.sqlite";

pub async fn initialize_db() -> anyhow::Result<SqlitePool> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Database created"),
            Err(e) => return Err(e.into()),
        }
    }

    let pool = SqlitePool::connect(DB_URL).await?;
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Database migrated"),
        Err(e) => return Err(e.into()),
    }

    Ok(pool)
}

#[derive(Debug, sqlx::FromRow)]
pub struct WeatherRow {
    pub id: i64,
    pub time: i64,
    pub interval: i64,
    pub temperature: f64,
}

#[derive(Debug, sqlx::FromRow)]
struct ApiKeyRow {
    id: i64,
    service_name: String,
    key: String,
}

pub async fn get_latest_weather_from_db(pool: &SqlitePool) -> anyhow::Result<Option<WeatherRow>> {
    let row = sqlx::query_as::<_, WeatherRow>(
        r#"
        SELECT id, time, interval, temperature
        FROM weather
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn save_weather_to_database(
    pool: &SqlitePool,
    weather_data: WeatherResponse,
) -> anyhow::Result<()> {
    let exists = get_latest_weather_from_db(pool).await?;
    if let Some(weather) = exists {
        let query = r#"
            UPDATE weather
            SET temperature = ?
            WHERE id = ?
        "#;

        sqlx::query(query)
            .bind(weather_data.current.temperature_2m)
            .bind(weather.id)
            .execute(pool)
            .await?;
    } else {
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
    }

    Ok(())
}

pub async fn get_api_key(
    pool: &SqlitePool,
    service_name: &str,
) -> anyhow::Result<Option<ApiKeyRow>> {
    let row = sqlx::query_as::<_, ApiKeyRow>(
        r#"
        SELECT id, service_name, key
        FROM api_keys
        WHERE service_name = ?
        "#,
    )
    .bind(service_name)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn save_api_key(pool: &SqlitePool, service_name: &str, key: &str) -> anyhow::Result<()> {
    let query = r#"
        INSERT INTO api_keys (service_name, key)
        VALUES (?, ?)
    "#;

    sqlx::query(query)
        .bind(service_name)
        .bind(key)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_api_key(pool: &SqlitePool, service_name: &str) -> anyhow::Result<()> {
    let query = r#"
        DELETE FROM api_keys
        WHERE service_name = ?
    "#;

    sqlx::query(query).bind(service_name).execute(pool).await?;

    Ok(())
}

pub async fn update_api_key(
    pool: &SqlitePool,
    service_name: &str,
    key: &str,
) -> anyhow::Result<()> {
    let query = r#"
        UPDATE api_keys
        SET key = ?
        WHERE service_name = ?
    "#;

    sqlx::query(query)
        .bind(key)
        .bind(service_name)
        .execute(pool)
        .await?;

    Ok(())
}

#[derive(sqlx::FromRow, Debug)]
pub struct OAuth2Token {
    pub app_name: String,
    pub client_secret: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: chrono::NaiveDateTime,
    pub code: String,
    pub get_token_url: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn get_token_from_db(
    pool: &SqlitePool,
    app_name: String,
) -> anyhow::Result<Option<OAuth2Token>> {
    let row = sqlx::query_as::<_, OAuth2Token>(
        r#"
        SELECT app_name, client_secret, client_id, redirect_uri, access_token, refresh_token, expires_at, code, get_token_url, created_at
        FROM oauth2_tokens
        WHERE app_name = ?
        "#,
    )
    .bind(app_name)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn add_new_oauth2_token_to_db(
    pool: &SqlitePool,
    data: OAuth2Token,
) -> anyhow::Result<()> {
    let exists = get_token_from_db(pool, data.app_name.clone()).await?;
    if let Some(_) = exists {
        let query = r#"
            UPDATE oauth2_tokens
            SET access_token = ?, refresh_token = ?, expires_at = ?, code = ?, get_token_url = ?, created_at = ?, client_secret = ?, client_id = ?, redirect_uri = ?
            WHERE app_name = ?
        "#;

        sqlx::query(query)
            .bind(data.access_token)
            .bind(data.refresh_token)
            .bind(data.expires_at)
            .bind(data.code)
            .bind(data.get_token_url)
            .bind(data.created_at)
            .bind(data.client_secret)
            .bind(data.client_id)
            .bind(data.redirect_uri)
            .bind(data.app_name)
            .execute(pool)
            .await?;
    } else {
        let query = r#"
        INSERT INTO oauth2_tokens (app_name, client_secret, client_id, redirect_uri, access_token, refresh_token, expires_at, code, get_token_url, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

        sqlx::query(query)
            .bind(data.app_name)
            .bind(data.client_secret)
            .bind(data.client_id)
            .bind(data.redirect_uri)
            .bind(data.access_token)
            .bind(data.refresh_token)
            .bind(data.expires_at)
            .bind(data.code)
            .bind(data.get_token_url)
            .bind(data.created_at)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn update_oauth2_access_and_refresh_tokens(
    pool: &SqlitePool,
    app_name: String,
    access_token: String,
    refresh_token: String,
    expires_at: chrono::NaiveDateTime,
) -> anyhow::Result<()> {
    let query = r#"
        UPDATE oauth2_tokens
        SET access_token = ?, refresh_token = ?, expires_at = ?
        WHERE app_name = ?
    "#;

    sqlx::query(query)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .bind(app_name)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_oauth2_token_from_db(
    pool: &SqlitePool,
    app_name: String,
) -> anyhow::Result<()> {
    let query = r#"
        DELETE FROM oauth2_tokens
        WHERE app_name = ?
    "#;

    sqlx::query(query).bind(app_name).execute(pool).await?;

    Ok(())
}
