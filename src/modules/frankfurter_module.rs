use crate::{application::ports::FXModule, domain::exceptions::DomainException, modules::dtos::ModuleCurrentCurrencyRateResponse};

pub struct FrankfurterModule {
    internal_cache: moka::future::Cache<String, String>
}

impl FrankfurterModule {
    pub fn new() -> FrankfurterModule {
        let internal_cache = moka::future::Cache::new(256);

        FrankfurterModule {
            internal_cache
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Provider {
    key: String,
    date: String,
    rate: f64,
    excluded: bool
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CurrencyRate {
    date: String,
    base: String,
    quote: String,
    rate: f64,
    providers: Option<Vec<Provider>>
}

#[async_trait::async_trait]
impl FXModule for FrankfurterModule {
    async fn get_current_rate(&self, request: &crate::modules::dtos::ModuleCurrentCurrencyRateRequest) -> Result<crate::modules::dtos::ModuleCurrentCurrencyRateResponse, DomainException> {
        let url = format!("https://api.frankfurter.dev/v2/rate/{}/{}", request.currency_a, request.currency_b);

        let resp = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(|err| {
                tracing::error!("Error occurred during fetching rate for remote pair: {} => {}: {}", request.currency_a, request.currency_b, err);
                DomainException::ModuleError("Cannot fetch remote rate for pair".to_string())
            })?
            .json::<CurrencyRate>()
            .await
            .map_err(|err| {
                tracing::error!("Cannot parse the rate: {}", err);
                DomainException::ModuleError("Cannot parse rate for pair".to_string())
            })?;

        return Ok(ModuleCurrentCurrencyRateResponse::new(resp.base.clone(), resp.quote.clone(), resp.rate)) 
    }
}
