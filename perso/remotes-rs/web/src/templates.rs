use std::{convert::Infallible, task::Context};

use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
};
use once_cell::sync::Lazy;
use tera::{Context, Tera};

pub struct Templates(pub str, pub Context);

impl Templates {
    fn get() -> &'static Tera {
        static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
            let mut tera = match Tera::new("templates/**/*") {
                Ok(t) => t,
                Err(e) => {
                    log::error!("Templates parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
            tera.autoescape_on(vec!["html", ".sql"]);
            // tera.register_filter("do_nothing", do_nothing_filter);
            tera
        });

        &TEMPLATES
    }
}

impl IntoResponse for Templates {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self.get().render(self.0, &self.1) {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                )))
                .unwrap(),
        }
    }
}
