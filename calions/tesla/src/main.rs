#[macro_use]
extern crate log;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

use actix_rt::System;
use applications::settings::Settings;
use std::io::Result as IoResult;
use std::sync::RwLock;

mod applications;
mod core;
mod domains;
mod dto;
mod infra;
mod tasks;
mod web;

lazy_static! {
    static ref SETTINGS: RwLock<Settings> =
        RwLock::new(Settings::new().expect("Unable to initialize settings"));
}

fn main() -> IoResult<()> {
    std::env::set_var("RUST_LOG", &Settings::get().log_level);
    env_logger::init();
    let system = System::new("App");
    web::executor::start();
    system.run()
}
