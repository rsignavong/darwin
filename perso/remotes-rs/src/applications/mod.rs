#[path = "config.rs"]
mod config;
mod error;

pub use self::config::Config;
pub use error::ApplicationError;

pub struct App {}

impl App {
    pub async fn start() -> Result<(), ApplicationError> {
        let cookie = &Settings::get().web.cookie;
        let listen = &Settings::get().web.listen;
        let repository = &Settings::get().repository;

        let services = Services::new(repository)?;

        Web::start(listen, cookie, services)?;

        Ok(())
    }
}
