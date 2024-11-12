use actix_files::Files;
use actix_web::web;

pub struct Assets;

impl Assets {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(Files::new("/static", "./tmp/static").use_last_modified(true));
    }
}
