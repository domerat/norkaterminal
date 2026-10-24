pub struct ModuleCurrentWeatherRequest {
    pub city: String
}

#[derive(serde::Serialize)]
pub struct ModuleCurrentWeatherResponse {
    pub current_weather: String,
    pub precipitation: f64,
    pub humidity: f64,
    pub temperature: f64,
}

pub struct ModuleCurrentCurrencyRateRequest {
    pub currency_a: String,
    pub currency_b: String
}

#[derive(serde::Serialize)]
pub struct ModuleCurrentCurrencyRateResponse {
    pub base: String,
    pub quote: String,
    pub rate: f64
}

impl ModuleCurrentWeatherRequest {
    pub fn new(city: String) -> ModuleCurrentWeatherRequest {
        ModuleCurrentWeatherRequest {
            city
        }
    }
}

impl ModuleCurrentWeatherResponse {
    pub fn new(current_weather: String, precipitation: f64, humidity: f64, temperature: f64) -> ModuleCurrentWeatherResponse {
        ModuleCurrentWeatherResponse {
            current_weather,
            precipitation,
            humidity,
            temperature
        }
    }
}

impl ModuleCurrentCurrencyRateRequest {
    pub fn new(currency_a: String, currency_b: String) -> ModuleCurrentCurrencyRateRequest {
        ModuleCurrentCurrencyRateRequest {
            currency_a,
            currency_b
        }
    }
}

impl ModuleCurrentCurrencyRateResponse {
    pub fn new(base: String, quote: String, rate: f64) -> ModuleCurrentCurrencyRateResponse {
        ModuleCurrentCurrencyRateResponse {
            base,
            quote,
            rate
        }
    }
}
