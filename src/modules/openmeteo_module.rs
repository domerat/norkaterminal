use crate::{application::ports::WeatherModule, domain::exceptions::DomainException::{self}};

pub struct OpenMeteoModule {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u32)]
enum WmoWeatherCode {
    ClearSky = 0,
    MainlyClear = 1,
    PartlyCloudy = 2,
    Overcast = 3,
    Fog = 45,
    DepositingRimeFog = 48,
    LightDrizzle = 51,
    ModerateDrizzle = 53,
    DenseDrizzle = 55,
    LightFreezingDrizzle = 56,
    DenseFreezingDrizzle = 57,
    SlightRain = 61,
    ModerateRain = 63,
    HeavyRain = 65,
    LightFreezingRain = 66,
    HeavyFreezingRain = 67,
    SlightSnowFall = 71,
    ModerateSnowFall = 73,
    HeavySnowFall = 75,
    SnowGrains = 77,
    SlightRainShowers = 80,
    ModerateRainShowers = 81,
    HeavyRainShowers = 82,
    SlightSnowShowers = 85,
    HeavySnowShowers = 86,
    Thunderstorm = 95,
    ThunderstormWithHail = 96,
    HeavyThunderstormWithHail = 99,
    Unknown,
}

impl From<i32> for WmoWeatherCode {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::ClearSky,
            1 => Self::MainlyClear,
            2 => Self::PartlyCloudy,
            3 => Self::Overcast,
            45 => Self::Fog,
            48 => Self::DepositingRimeFog,
            51 => Self::LightDrizzle,
            53 => Self::ModerateDrizzle,
            55 => Self::DenseDrizzle,
            56 => Self::LightFreezingDrizzle,
            57 => Self::DenseFreezingDrizzle,
            61 => Self::SlightRain,
            63 => Self::ModerateRain,
            65 => Self::HeavyRain,
            66 => Self::LightFreezingRain,
            67 => Self::HeavyFreezingRain,
            71 => Self::SlightSnowFall,
            73 => Self::ModerateSnowFall,
            75 => Self::HeavySnowFall,
            77 => Self::SnowGrains,
            80 => Self::SlightRainShowers,
            81 => Self::ModerateRainShowers,
            82 => Self::HeavyRainShowers,
            85 => Self::SlightSnowShowers,
            86 => Self::HeavySnowShowers,
            95 => Self::Thunderstorm,
            96 => Self::ThunderstormWithHail,
            99 => Self::HeavyThunderstormWithHail,
            _ => Self::Unknown,
        }
    }
}

impl WmoWeatherCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ClearSky => "Clear sky",
            Self::MainlyClear => "Mainly clear",
            Self::PartlyCloudy => "Partly cloudy",
            Self::Overcast => "Overcast",
            Self::Fog => "Fog",
            Self::DepositingRimeFog => "Depositing rime fog",
            Self::LightDrizzle => "Light drizzle",
            Self::ModerateDrizzle => "Moderate drizzle",
            Self::DenseDrizzle => "Dense drizzle",
            Self::LightFreezingDrizzle => "Light freezing drizzle",
            Self::DenseFreezingDrizzle => "Dense freezing drizzle",
            Self::SlightRain => "Slight rain",
            Self::ModerateRain => "Moderate rain",
            Self::HeavyRain => "Heavy rain",
            Self::LightFreezingRain => "Light freezing rain",
            Self::HeavyFreezingRain => "Heavy freezing rain",
            Self::SlightSnowFall => "Slight snow fall",
            Self::ModerateSnowFall => "Moderate snow fall",
            Self::HeavySnowFall => "Heavy snow fall",
            Self::SnowGrains => "Snow grains",
            Self::SlightRainShowers => "Slight rain showers",
            Self::ModerateRainShowers => "Moderate rain showers",
            Self::HeavyRainShowers => "Heavy rain showers",
            Self::SlightSnowShowers => "Slight snow showers",
            Self::HeavySnowShowers => "Heavy snow showers",
            Self::Thunderstorm => "Thunderstorm",
            Self::ThunderstormWithHail => "Thunderstorm with slight hail",
            Self::HeavyThunderstormWithHail => "Thunderstorm with heavy hail",
            Self::Unknown => "Unknown weather code",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LatitudeAndLongtude {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CurrentWeather {
    time: String,
    interval: u64,
    temperature_2m: f64,
    relative_humidity_2m: u64,
    apparent_temperature: f64,
    precipitation: f64,
    rain: f64,
    showers: f64,
    snowfall: f64,
    weather_code: WmoWeatherCode
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CurrentGeodataResponse {
    results: Vec<LatitudeAndLongtude>
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CurrentWeatherAPIResponse {
    current: CurrentWeather
}

async fn get_latitude(name: &str) -> Result<Vec<LatitudeAndLongtude>, DomainException> {
    let reqwest_client = reqwest::Client::new();

    tracing::info!("Searching for latitude and longitude for {}", name);

    let response = reqwest_client
        .get("https://geocoding-api.open-meteo.com/v1/search")
        .query(&[
            ("name", name)
        ])
        .send()
        .await
        .map_err(|err| {
            tracing::error!("Cannot connect to the Open-Meteo API: {}", err);
            DomainException::ModuleError("Cannot connect to the Open-Meteo API.".to_string())
        })?
        .json::<CurrentGeodataResponse>()
        .await
        .map_err(|err| {
            tracing::error!("Cannot parse the result from geocoding api: {}", err);
            DomainException::ModuleError("Cannot parse result!".to_string())
        })?;

    tracing::info!("Successfully found coordinates!");

    Ok(response.results)
}

impl OpenMeteoModule {
    pub fn new() -> OpenMeteoModule {
        OpenMeteoModule { }
    }
}

#[async_trait::async_trait]
impl WeatherModule for OpenMeteoModule {
        async fn get_current_weather(&self, request: &crate::modules::dtos::ModuleCurrentWeatherRequest) -> Result<crate::modules::dtos::ModuleCurrentWeatherResponse, DomainException> {
        let geodata = get_latitude(request.city.as_str()).await?;

        match geodata.first() {
            Some(p) => {
               tracing::info!("Getting weather info from the Open-Meteo...");

               let response = reqwest::Client::new()
                    .get("https://api.open-meteo.com/v1/forecast")
                    .query(&[
                        ("latitude", format!("{}", p.latitude).as_str()),
                        ("longitude", format!("{}", p.longitude).as_str()),
                        ("current", "temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,rain,showers,snowfall,weather_code")
                    ])
                    .send()
                    .await
                    .map_err(|err| {
                        tracing::error!("Cannot fetch the weather from the Open Meteo API: {}", err);
                        DomainException::ModuleError("Cannot fetch the weather from the Open Meteo API!".to_string())
                    })?
                    .json::<CurrentWeatherAPIResponse>()
                    .await
                    .map_err(|err| {
                        tracing::error!("Cannot parse the weather from the Open Meteo API: {err:?}");
                        DomainException::ModuleError("Cannot parse the weather from the Open Meteo API!".to_string())
                    })?;

                tracing::info!("Responding with weather...");

                Ok(crate::modules::dtos::ModuleCurrentWeatherResponse::new(
                        response.current.weather_code.as_str().to_string(),
                        response.current.precipitation,
                        response.current.relative_humidity_2m as f64,
                        response.current.temperature_2m
                ))
            },
            None => {
                Err(DomainException::ModuleError("No data for this city has been found, or the city is not found.".to_string()))
            }
        }
    }
}
