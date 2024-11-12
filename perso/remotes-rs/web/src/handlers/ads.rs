use crate::{Templates, WebError};
use actix_session::Session;
use actix_web::web::{block, Data, Json};
use actix_web::{HttpResponse, Result};
use axum::response::IntoResponse;
use resources::commands::RecruiterJobCommand;
use resources::entities::{CompanyId, UserId};
use services::{RecruiterJob, Services};
use tera::Context;

pub struct Ads;

impl Ads {
    pub async fn create(
        services: Data<Services>,
        session: Session,
        command: Json<RecruiterJobCommand>,
    ) -> impl IntoResponse {
        let conn = services.postgresql.pool()?;
        let user_id = session.get::<UserId>("user_id")?;

        let job = block(move || RecruiterJob::create(&conn, user_id, command.into_inner())).await?;

        Ok(HttpResponse::Ok().json(job))
    }

    pub async fn update(
        services: Data<Services>,
        session: Session,
        command: Json<RecruiterJobCommand>,
    ) -> impl IntoResponse {
        let conn = services.postgresql.pool()?;
        let user_id = session.get::<UserId>("user_id")?;

        let job = block(move || RecruiterJob::create(&conn, user_id, command.into_inner())).await?;

        Ok(HttpResponse::Ok().json(job))
    }
}
