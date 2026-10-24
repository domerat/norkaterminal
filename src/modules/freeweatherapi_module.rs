use crate::application::ports::WeatherModule;
use crate::domain::exceptions::DomainException;
use crate::modules::dtos::ModuleCurrentWeatherResponse;

pub struct FreeWeatherAPI {
    token: String
}

impl FreeWeatherAPI {
    pub fn new(token: &str) -> FreeWeatherAPI {
        FreeWeatherAPI {
            token: token.to_string()
        }
    }
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    #[serde(rename = "last_updated_epoch")]
    pub last_updated_epoch: i64,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "temp_c")]
    pub temp_c: f64,
    #[serde(rename = "temp_f")]
    pub temp_f: f64,
    #[serde(rename = "is_day")]
    pub is_day: i64,
    pub condition: Condition,
    #[serde(rename = "wind_mph")]
    pub wind_mph: f64,
    #[serde(rename = "wind_kph")]
    pub wind_kph: f64,
    #[serde(rename = "wind_degree")]
    pub wind_degree: i64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mb")]
    pub pressure_mb: f64,
    #[serde(rename = "pressure_in")]
    pub pressure_in: f64,
    #[serde(rename = "precip_mm")]
    pub precip_mm: f64,
    #[serde(rename = "precip_in")]
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    #[serde(rename = "feelslike_c")]
    pub feelslike_c: f64,
    #[serde(rename = "feelslike_f")]
    pub feelslike_f: f64,
    #[serde(rename = "windchill_c")]
    pub windchill_c: f64,
    #[serde(rename = "windchill_f")]
    pub windchill_f: f64,
    #[serde(rename = "heatindex_c")]
    pub heatindex_c: f64,
    #[serde(rename = "heatindex_f")]
    pub heatindex_f: f64,
    #[serde(rename = "dewpoint_c")]
    pub dewpoint_c: f64,
    #[serde(rename = "dewpoint_f")]
    pub dewpoint_f: f64,
    #[serde(rename = "vis_km")]
    pub vis_km: f64,
    #[serde(rename = "vis_miles")]
    pub vis_miles: f64,
    pub uv: f64,
    #[serde(rename = "gust_mph")]
    pub gust_mph: f64,
    #[serde(rename = "gust_kph")]
    pub gust_kph: f64,
    #[serde(rename = "will_it_rain")]
    pub will_it_rain: i64,
    #[serde(rename = "chance_of_rain")]
    pub chance_of_rain: i64,
    #[serde(rename = "will_it_snow")]
    pub will_it_snow: i64,
    #[serde(rename = "chance_of_snow")]
    pub chance_of_snow: i64,
    #[serde(rename = "wetbulb_c")]
    pub wetbulb_c: f64,
    #[serde(rename = "wetbulb_f")]
    pub wetbulb_f: f64,
    #[serde(rename = "short_rad")]
    pub short_rad: f64,
    #[serde(rename = "diff_rad")]
    pub diff_rad: f64,
    pub dni: f64,
    pub gti: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerResponse {
    current: WeatherResponse
}

#[async_trait::async_trait]
impl WeatherModule for FreeWeatherAPI {
    async fn get_current_weather(&self, request: &crate::modules::dtos::ModuleCurrentWeatherRequest) -> Result<crate::modules::dtos::ModuleCurrentWeatherResponse, DomainException> { 
        tracing::info!("Getting weather from FreeWeatherAPI for city: {}", request.city);

        let requ = reqwest::Client::new()
            .get("https://api.weatherapi.com/v1/current.json")
            .query(&[
                ("key".to_string(), &self.token),
                ("q".to_string(), &request.city),
                ("aqi".to_string(), &"no".to_string())
            ])
            .send()
            .await
            .map_err(|err| {
                tracing::error!("Error occurred during fetching WeatherAPI data: {}", err);
                DomainException::ModuleError("Cannot fetch WeatherAPI data".to_string())
            })?
            .json::<ServerResponse>()
            .await
            .map_err(|err| {
                tracing::error!("Error occurred during parsing WeatherAPI data: {}", err);
                DomainException::ModuleError("Cannot parse WeatherAPI data".to_string())
            })?;

        tracing::info!("Got results from FreeWeatherAPI for city: {}!", request.city);

        Ok(ModuleCurrentWeatherResponse::new(
            requ.current.condition.text,
            requ.current.precip_mm,
            requ.current.humidity as f64,
            requ.current.temp_c
        ))
    }
}
