use crate::{Templates, WebError};
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, Result};
use services::Services;
use tera::Context;

pub struct Apps;

impl Apps {
    pub async fn show(services: Data<Services>) -> Result<HttpResponse> {
        let body = Templates::get()
            .render("web/app.html", &Context::new())
            .map_err(|source| WebError::TemplateRender { source })?;

        Ok(HttpResponse::Ok().body(body))
    }
}
