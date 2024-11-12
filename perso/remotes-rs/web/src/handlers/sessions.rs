use axum::response::IntoResponse;

pub struct Sessions;

impl Sessions {
    pub async fn create() -> impl IntoResponse {
        "Welcome!"
    }

    pub async fn show() -> impl IntoResponse {
        "Session!"
    }
}
