pub struct DBCreateNewUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub name: String,
    pub group: String
}

pub struct DBCreateNewUserResponse {
    pub id: String
}

pub struct DBGetUserRequest {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>
}

pub struct DBGetUserResponse {
    pub username: String,
    pub password: String,
    pub email: String,
    pub name: String,
    pub group: String
}

impl DBCreateNewUserRequest {
    pub fn new(username: String, password: String, email: String, name: String, group: String) -> DBCreateNewUserRequest {
        DBCreateNewUserRequest {
            username,
            password,
            email,
            name,
            group
        }
    }
}

impl DBCreateNewUserResponse {
    pub fn new(id: String) -> DBCreateNewUserResponse {
        DBCreateNewUserResponse {
            id
        }
    }
}

impl DBGetUserRequest {
    pub fn new(id: Option<String>, username: Option<String>, email: Option<String>) -> DBGetUserRequest {
        DBGetUserRequest {
            id,
            username,
            email
        }
    }
}

impl DBGetUserResponse {
    pub fn new(username: String, password: String, email: String, name: String, group: String) -> DBGetUserResponse {
        DBGetUserResponse {
            username,
            password,
            email,
            name,
            group
        }
    }
}
