#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_new;

mod applications;
mod encoders;
mod processors;
mod producers;
mod resources;
mod wizard;

use applications::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if let Err(e) = App::start().await {
        error!("App: {}", e);
    }

    Ok(())
}
