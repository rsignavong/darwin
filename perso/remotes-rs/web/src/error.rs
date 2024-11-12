use actix_web::ResponseError;
use std::io::Error as IoError;
use tera::Error as TeraError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebError {
    #[error("CookiePrivateKeyInvalidLength")]
    CookiePrivateKeyInvalidLength,
    #[error("HttpServer")]
    HttpServer(#[from] IoError),
    #[error("TemplateContext (source {source:?})")]
    TemplateContext { source: TeraError },
    #[error("TemplateRender (source {source:?})")]
    TemplateRender { source: TeraError },
}

impl ResponseError for WebError {}
