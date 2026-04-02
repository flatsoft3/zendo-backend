use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status: u16,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, String>>
}

//helper methods

impl<T: Serialize> ApiResponse<T> {
    pub fn success(description: impl Into<String>, data: Option<T>) -> Self {
        Self {
            status: 200,
            description: description.into(),
            data,
            errors: None
        }
    }

    pub fn error(status: u16, description: impl Into<String>) -> Self {
        Self {
            status,
            description: description.into(),
            data: None,
            errors: None
        }
    }


    pub fn validation_error(status: u16, description: impl Into<String>, errors: HashMap<String, String>) -> Self {
        Self {
            status,
            description: description.into(),
            data: None,
            errors: Some(errors)
        }
    }
}


