use crate::{
    application::{dto::{AppAuthenticationUserRequest, AppCommandRequest}, ports::{InboundPort}}, 
    infrastructure::adapters::inbound::dto::{
    HTTPException, InboundAuthenticationRequest, InboundAuthenticationResponse, InboundCreateNewUserRequest, InboundCreateNewUserResponse, InboundIssueCommandRequest, InboundIssueCommandResponse
}};

#[derive(Clone)]
pub struct HTTPServer {
    core: std::sync::Arc<crate::application::app::Core>,
}

impl HTTPServer {
    pub fn new(core: std::sync::Arc<crate::application::app::Core>) -> Self {
        Self {
            core
        } 
    }

    pub fn create(self) -> axum::Router {
        axum::Router::new()
            .route("/command", axum::routing::post(execute_command))
            .route("/api/admin/register", axum::routing::post(register_user))
            .route("/api/admin/login", axum::routing::post(authenticate_user))
            .with_state(self)
            .layer(axum::middleware::from_fn(crate::middlewares::http_logging_middleware::http_logging_middleware))    
    }
}

async fn execute_command(
    axum::extract::State(state): axum::extract::State<HTTPServer>,
    axum::Json(data): axum::Json<InboundIssueCommandRequest>
) -> Result<axum::Json<InboundIssueCommandResponse>, HTTPException> {
    let p = state.core.command(AppCommandRequest::new(data.command)).await
        .map_err(|err| {
            println!("Cannot execute command: {}", err);
            HTTPException::IncorrectCommand
        })?;

    Ok(axum::Json(InboundIssueCommandResponse {data: p.result}))
}

async fn register_user(
    axum::extract::State(state): axum::extract::State<HTTPServer>,
    axum::Json(data): axum::Json<InboundCreateNewUserRequest>
) -> Result<axum::Json<InboundCreateNewUserResponse>, HTTPException> { 

    match state.core.register(
        crate::application::dto::AppCreateNewUserRequest::new(data.username, data.password, data.email, data.name)
    ).await {
        Ok(resp) => {
            Ok(axum::Json(InboundCreateNewUserResponse::new(resp.id)))
        }

        Err(err) => {
            tracing::error!("Cannot sign up user (layer 0): {}", err);
            Err(HTTPException::CannotSignUp)
        }
    }
}

async fn authenticate_user(
    axum::extract::State(state): axum::extract::State<HTTPServer>,
    axum::Json(data): axum::Json<InboundAuthenticationRequest>
) -> Result<axum::Json<InboundAuthenticationResponse>, HTTPException> {
    match state.core.authenticate(AppAuthenticationUserRequest::new(data.username, data.password)).await {
        Ok(p) => {
            Ok(axum::Json(InboundAuthenticationResponse::new(p.session)))
        }

        Err(err) => {
            tracing::error!("Cannot authenticate user (layer 0): {}", err);
            Err(HTTPException::CannotLogin)
        }
    }
}
