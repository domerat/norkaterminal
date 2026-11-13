use argon2::PasswordVerifier;
use rand::TryRng;
use base64::Engine;
use argon2::PasswordHasher;
use crate::application::dto::{AppAuthenticationUserResponse, AppCommandRequest, AppCommandResponse, AppCreateNewUserResponse};
use crate::application::parser::CommandTypes;
use crate::infrastructure::adapters::outbound::dto::DBGetUserRequest;
use crate::{application::ports::InboundPort, infrastructure::adapters::outbound::user::UserAdapter};
use crate::application::ports::{FXModule, OutboundUserPort, WeatherModule};
use crate::domain::exceptions::DomainException;

#[derive(Clone)]
pub struct Core {
    users_outbound_adapter: UserAdapter,

    weather_modules: Vec<std::sync::Arc<dyn WeatherModule>>,
    fx_modules: Vec<std::sync::Arc<dyn FXModule>>
}

impl Core {
    pub fn new(
        users_outbound_adapter: UserAdapter, 
        weather_modules: Vec<std::sync::Arc<dyn WeatherModule>>,
        fx_modules: Vec<std::sync::Arc<dyn FXModule>>
    ) -> Self {
        Self {
            users_outbound_adapter,
            weather_modules,
            fx_modules
        }
    }
}

#[async_trait::async_trait]
impl InboundPort for Core {
    async fn command(&self, command: AppCommandRequest) -> Result<AppCommandResponse, DomainException> {
        let parser = crate::application::parser::Parser::new();
        let result = parser.parse(command.query)?;        

        match result.command {
            CommandTypes::WEATHER => {
                let city = result.args.first().cloned().unwrap_or_else(|| "Almaty".to_string());
                
                tracing::info!("Responding for {}", city);

                let responses = crate::application::helpers::fetch_weather_from_modules(&self.weather_modules, &city).await?;
                
                Ok(AppCommandResponse::new(responses))
            },

            CommandTypes::FX => {
                let currency_a = result.args.first().cloned().unwrap_or_else(|| "USD".to_string());
                let currency_b = result.args.get(1).cloned().unwrap_or_else(|| "CHF".to_string());
                let amount = result.args.get(2).cloned().unwrap_or_else(|| "1".to_string()).parse::<f64>().unwrap_or(1.0);

                let mut responses = crate::application::helpers::fetch_currency_rate_from_modules(&self.fx_modules, &currency_a, &currency_b).await?;

                responses.iter_mut().for_each(|v| {
                    v["total"] = serde_json::Value::from(v["rate"].as_f64().unwrap() * amount);
                });

                Ok(AppCommandResponse::new(responses))
            },

            _ => {
                Err(DomainException::UnknownCommand)
            }
        }
    }

    // WHY THERE'S NO AUTHENTICATION MANAGER?
    // NOT BECAUSE I'M LAZY, BUT BECAUSE I DON'T WANT TO WASTE MY TIME ON THIS NOW, I CAN FIX IT LATER.
    // AND YEAH, I KNOW ABOUT THIS BECAUSE THIS FILE IS OPEN 24/7 IN THE NVIM.
    async fn register(&self, user_request: crate::application::dto::AppCreateNewUserRequest) -> 
       Result<crate::application::dto::AppCreateNewUserResponse, DomainException> 
    {
        let argon2_engine = argon2::Argon2::default(); 
        
        let password_hash_ = argon2_engine.hash_password(&user_request.password.into_bytes())
            .map_err(|err| {
                tracing::error!("Cannot generate user hash on layer-2: {}", err);
                DomainException::RegistrationFailure
            })?;

        let user = match self.users_outbound_adapter.create_user(
            crate::infrastructure::adapters::outbound::dto::DBCreateNewUserRequest::new(
               user_request.username,
               password_hash_.to_string(),
               user_request.email,
               user_request.name,
               "user".to_string()
            )
        ).await {
            Ok(user) => {
                user
            }

            Err(err) => {
                return Err(err)
            }
        };

        Ok(AppCreateNewUserResponse {
            id: user.id
        })
    }

     async fn authenticate(&self, auth_request: crate::application::dto::AppAuthenticationUserRequest) -> Result<crate::application::dto::AppAuthenticationUserResponse, DomainException> {
        let user = self.users_outbound_adapter.get_user(DBGetUserRequest::new(None, Some(auth_request.username), None)).await
            .map_err(|err| {
                tracing::error!("Error occurred on layer-2: {}", err);
                err
            })?;

        let argon2_engine = argon2::Argon2::default();
        let password_hash = password_hash::phc::PasswordHash::new(&user.password).expect("Database is probably corrupted.");
        
        match argon2_engine.verify_password(&auth_request.password.into_bytes(), &password_hash) {
            Ok(_) => {
                let mut rand_engine = rand::rngs::SysRng;
                let mut session = [0u8; 64];

                let _ = rand_engine.try_fill_bytes(&mut session)
                    .map_err(|err| {
                        tracing::error!("Cannot generate the session on layer-2! Error: {}", err);
                        DomainException::AuthenticationFailure
                    })?;

                let total_token = base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(session);

                Ok(AppAuthenticationUserResponse::new(total_token))
            }

            Err(err) => {
                tracing::error!("Error occured on layer-2 during password verification: {}", err);
                return Err(DomainException::AuthenticationFailure)
            }
        }
    }
    
    async fn logoff(&self) {
        // truly? Idk what to write here. It's JWTs! 
    }
}


