use crate::domain::exceptions::DomainException;

pub async fn fetch_weather_from_modules(
    modules: &[std::sync::Arc<dyn crate::application::ports::WeatherModule>],
    city: &str,
) -> Result<Vec<serde_json::Value>, DomainException> {
    use futures::StreamExt;

    let mut futures = Vec::with_capacity(modules.len());
    for module in modules {
        let module = module.clone();
        let city = city.to_string();
        
        futures.push(async move {
            module.get_current_weather(
                &crate::modules::dtos::ModuleCurrentWeatherRequest::new(city)
            ).await
        });
    }

    let mut results = futures::stream::iter(futures).buffer_unordered(10);
    let mut responses = vec![];

    while let Some(res) = results.next().await {
        match res {
            Ok(data) => {
                if let Ok(json_data) = serde_json::to_value(data) {
                    responses.push(json_data);
                }
            }
            Err(err) => {
                tracing::error!("{}", err);
                return Err(err)
            }
        }
    }

    Ok(responses)
}

pub async fn fetch_currency_rate_from_modules(
    modules: &[std::sync::Arc<dyn crate::application::ports::FXModule>],
    currency_a: &str,
    currency_b: &str,
) -> Result<Vec<serde_json::Value>, DomainException> {
    use futures::StreamExt;

    let mut futures = Vec::with_capacity(modules.len());
    for module in modules {
        let module = module.clone(); 
        
        futures.push(async move {
            module.get_current_rate(
                &crate::modules::dtos::ModuleCurrentCurrencyRateRequest::new(currency_a.to_string(), currency_b.to_string())
            ).await
        });
    }

    let mut results = futures::stream::iter(futures).buffer_unordered(10);
    let mut responses = vec![];

    while let Some(res) = results.next().await {
        match res {
            Ok(data) => {
                if let Ok(json_data) = serde_json::to_value(data) {
                    responses.push(json_data);
                }
            }
            Err(err) => {
                tracing::error!("{}", err);
                return Err(err)
            }
        }
    }

    Ok(responses)
}
