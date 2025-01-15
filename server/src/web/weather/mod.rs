use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::tcp::StateMessage;

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

pub async fn get_weather_data() -> anyhow::Result<WeatherResponse> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=54.3523&longitude=18.6491&current=temperature_2m&timezone=Europe%2FBerlin";
    let response = reqwest::get(url).await?;
    let data = response.json::<WeatherResponse>().await?;
    Ok(data)
}

pub async fn weather_polling_task(
    sender: mpsc::Sender<StateMessage>,
) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
        interval.tick().await;
        let weather_data = get_weather_data().await?;
        
        let payload = format!(
            "Weather: {}°C",
            weather_data.current.temperature_2m
        );

        sender.send(StateMessage::WeatherData(payload)).await?;
        
    }
}