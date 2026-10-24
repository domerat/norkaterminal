use axum::response::IntoResponse;

#[derive(serde::Deserialize)]
pub struct InboundIssueCommandRequest {
    pub command: String
}

#[derive(serde::Serialize)]
pub struct InboundIssueCommandResponse
{
    pub data: Vec<serde_json::Value>
}

#[derive(serde::Deserialize)]
pub struct InboundCreateNewUserRequest {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: String
}

#[derive(serde::Serialize)]
pub struct InboundCreateNewUserResponse {
    id: String
}

#[derive(serde::Deserialize)]
pub struct InboundAuthenticationRequest {
    pub username: String,
    pub password: String
}

#[derive(serde::Serialize)]
pub struct InboundAuthenticationResponse {
    pub session: String
}

impl InboundIssueCommandRequest {
    pub fn new(command: String) -> InboundIssueCommandRequest {
        InboundIssueCommandRequest {
            command
        }
    }
}

impl InboundIssueCommandResponse {
    pub fn new(data: Vec<serde_json::Value>) -> InboundIssueCommandResponse {
        InboundIssueCommandResponse {
            data
        }
    }
}

impl InboundCreateNewUserRequest {
    pub fn new(username: String, password: String, name: String, email: String) -> InboundCreateNewUserRequest {
        InboundCreateNewUserRequest {
            username,
            password,
            name,
            email
        }
    }
}

impl InboundCreateNewUserResponse {
    pub fn new(id: String) -> InboundCreateNewUserResponse {
        InboundCreateNewUserResponse {
            id
        }
    }
}

impl InboundAuthenticationRequest {
    pub fn new(username: String, password: String) -> InboundAuthenticationRequest {
        InboundAuthenticationRequest {
            username,
            password
        }
    }
}

impl InboundAuthenticationResponse {
    pub fn new(session: String) -> InboundAuthenticationResponse {
        InboundAuthenticationResponse {
            session
        }
    }
}

// ------------ EXCEPTIONS 

#[derive(serde::Serialize)]
pub enum HTTPException {
    IncorrectCommand,
    CannotSignUp,
    CannotLogin
}

#[derive(serde::Serialize)]
pub struct HTTPExceptionResponse {
    error: HTTPException,
    message: String
}

impl IntoResponse for HTTPException {
   fn into_response(self) -> axum::response::Response {
        match self {
            Self::IncorrectCommand => 
                (axum::http::StatusCode::BAD_REQUEST, axum::Json(
                    HTTPExceptionResponse {
                        error: Self::IncorrectCommand, 
                        message: "Incorrect command or arguments.".to_string()
                    })
            ).into_response(),

            Self::CannotSignUp =>
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::Json(
                    HTTPExceptionResponse {
                        error: Self::CannotSignUp,
                        message: "Cannot sign up you right now.".to_string()
                    }
                )
            ).into_response(),

            Self::CannotLogin => 
                (axum::http::StatusCode::UNAUTHORIZED, axum::Json(
                    HTTPExceptionResponse {
                        error: Self::CannotLogin,
                        message: "Cannot login you right now.".to_string()
                    }
                )
            ).into_response()
        }
    } 
}
