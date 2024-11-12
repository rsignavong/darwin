use crate::web::handlers::status;
use actix_web::{dev::HttpServiceFactory, web};

pub fn init() -> impl HttpServiceFactory {
    web::scope("/status").route("", web::to(status::show))
}
