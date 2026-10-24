use crate::{application::ports::{FXModule, WeatherModule}, infrastructure::{self, adapters::inbound::{http_server::HTTPServer, ssh::SSHInbound}}};

pub struct Application {
    pub inbound_http_adapter: HTTPServer,
    pub inbound_ssh_adapter: SSHInbound,
    pub inbound_http_listener: tokio::net::TcpListener,
    pub inbound_ssh_listener: std::net::SocketAddr,
    pub inbound_ssh_configuration: russh::server::Config
}

pub async fn bootstrap() -> Result<Application, Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_thread_names(true)
        .init();

    dotenvy::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite::memory:".to_string());

    let freeweatherapi_token = std::env::var("FREEWEATHERAPI_TOKEN")
        .expect("FreeWeatherAPI requires a token.");

    let listening_url_http = std::env::var("HTTP_LISTENING_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:9000".to_string());

    let listening_url_ssh = std::env::var("SSH_LISTENING_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:2222".to_string());

    let database_connection_options = sea_orm::ConnectOptions::new(database_url);
    let database_pool = sea_orm::Database::connect(database_connection_options).await?;

    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![
            russh::keys::PrivateKey::random(&mut rand::rng(), russh::keys::Algorithm::Ed25519)
                .unwrap(),
        ], 
        ..Default::default()
    };

    let mut weather_modules : Vec<std::sync::Arc<dyn WeatherModule>> = vec![];
    let mut fx_modules : Vec<std::sync::Arc<dyn FXModule>> = vec![];

    let http_tcp_listener = tokio::net::TcpListener::bind(listening_url_http).await?;
    let ssh_tcp_listener = listening_url_ssh.parse()?;

    let open_meteo_weather_module = crate::modules::openmeteo_module::OpenMeteoModule::new();
    let freeweatherapi_module = crate::modules::freeweatherapi_module::FreeWeatherAPI::new(&freeweatherapi_token);
    let frankfurter_module = crate::modules::frankfurter_module::FrankfurterModule::new();

    weather_modules.push(std::sync::Arc::new(open_meteo_weather_module));
    weather_modules.push(std::sync::Arc::new(freeweatherapi_module));
    fx_modules.push(std::sync::Arc::new(frankfurter_module));

    let outbound_users_adapter = crate::infrastructure::adapters::outbound::user::UserAdapter::new(database_pool);
    let core = crate::application::app::Core::new(outbound_users_adapter, weather_modules, fx_modules);

    let inbound_http_adapter = crate::infrastructure::adapters::inbound::http_server::HTTPServer::new(std::sync::Arc::from(core.clone()));
    let inbound_ssh_adapter = crate::infrastructure::adapters::inbound::ssh::SSHInbound::new(std::sync::Arc::from(core.clone()));

    Ok(
        Application { 
            inbound_http_adapter: inbound_http_adapter,
            inbound_ssh_adapter: inbound_ssh_adapter,
            inbound_http_listener: http_tcp_listener,
            inbound_ssh_listener: ssh_tcp_listener,
            inbound_ssh_configuration: config
        }
    )
}

