use actix_web::web::{block, Data};
use actix_web::{HttpResponse, Result};
use services::{Services, Status as StatusService};

pub struct Status;

impl Status {
    pub async fn show(services: Data<Services>) -> Result<HttpResponse> {
        let conn = services.postgresql.pool()?;
        let status = block(move || StatusService::get(&conn)).await?;
        Ok(HttpResponse::Ok().json(status))
    }
}
