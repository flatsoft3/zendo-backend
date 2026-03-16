use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status: u16,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

//helper methods

impl<T: Serialize> ApiResponse<T> {
    pub fn success(description: impl Into<String>, data: Option<T>) -> Self {
        Self {
            status: 200,
            description: description.into(),
            data: data,
        }
    }

    pub fn error(status: u16, description: impl Into<String>) -> Self {
        Self {
            status: status,
            description: description.into(),
            data: None,
        }
    }
}


