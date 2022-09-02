use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseReason {
    UsernameAlreadyExists,
    UsernameNotFound,
    InvalidUsernameOrPassword,
    InternalError,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub reason: Option<ResponseReason>,
}

impl Response {
    pub fn bad(reason: ResponseReason) -> Self {
        Self {
            success: false,
            reason: Some(reason),
        }
    }

    pub fn good() -> Self {
        Self {
            success: true,
            reason: None,
        }
    }
}
