use crate::domain::exceptions::DomainException;
use crate::infrastructure::adapters::outbound::dto::{
    DBCreateNewUserRequest,
    DBGetUserRequest,
    DBCreateNewUserResponse,
    DBGetUserResponse
};

#[async_trait::async_trait]
pub trait InboundPort : Send + Sync {
    async fn command(&self, command_request: crate::application::dto::AppCommandRequest) -> Result<crate::application::dto::AppCommandResponse, DomainException>;
    async fn authenticate(&self, auth_request: crate::application::dto::AppAuthenticationUserRequest) -> Result<crate::application::dto::AppAuthenticationUserResponse, DomainException>;
    async fn register(&self, user_request: crate::application::dto::AppCreateNewUserRequest) -> Result<crate::application::dto::AppCreateNewUserResponse, DomainException>;
    async fn logoff(&self);
}

#[async_trait::async_trait]
pub trait OutboundUserPort : Send + Sync {
    async fn create_user(&self, data: DBCreateNewUserRequest) -> Result<DBCreateNewUserResponse, DomainException>;
    async fn get_user(&self, data: DBGetUserRequest) -> Result<DBGetUserResponse, DomainException>;
}

#[async_trait::async_trait]
pub trait WeatherModule : Send + Sync {
    async fn get_current_weather(&self, request: &crate::modules::dtos::ModuleCurrentWeatherRequest) -> Result<crate::modules::dtos::ModuleCurrentWeatherResponse, DomainException>;
}

#[async_trait::async_trait]
pub trait FXModule : Send + Sync {
    async fn get_current_rate(&self, request: &crate::modules::dtos::ModuleCurrentCurrencyRateRequest) -> Result<crate::modules::dtos::ModuleCurrentCurrencyRateResponse, DomainException>;
}
