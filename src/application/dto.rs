pub struct AppCreateNewUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub name: String
}

pub struct AppCreateNewUserResponse {
    pub id: String
}

pub struct AppAuthenticationUserRequest {
    pub username: String,
    pub password: String
}

pub struct AppAuthenticationUserResponse {
    pub session: String
}

pub struct AppCommandRequest {
    pub query: String
}

#[derive(Debug)]
pub struct AppCommandResponse {
    pub result: Vec<serde_json::Value>
}

impl AppCreateNewUserRequest {
    pub fn new(username: String, password: String, email: String, name: String) -> AppCreateNewUserRequest {
        AppCreateNewUserRequest {
            username,
            password,
            email,
            name
        }
    }
}

impl AppCreateNewUserResponse {
    pub fn new(id: String) -> AppCreateNewUserResponse {
        AppCreateNewUserResponse {
            id
        }
    }
}

impl AppAuthenticationUserRequest {
    pub fn new(username: String, password: String) -> AppAuthenticationUserRequest {
        AppAuthenticationUserRequest {
            username,
            password
        }
    }
}

impl AppAuthenticationUserResponse {
    pub fn new(session: String) -> AppAuthenticationUserResponse {
        AppAuthenticationUserResponse {
            session
        }
    }
}

impl AppCommandRequest {
    pub fn new(query: String) -> AppCommandRequest {
        AppCommandRequest {query}
    }
}

impl AppCommandResponse {
    pub fn new (result: Vec<serde_json::Value>) -> AppCommandResponse {
        AppCommandResponse {result}
    }
}
