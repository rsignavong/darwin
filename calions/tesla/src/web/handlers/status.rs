use actix_web::{HttpRequest, HttpResponse, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct Status<'a> {
    status: &'a str,
}

pub fn show(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Status { status: "Ok" }))
}
