mod applications;
mod consumers;
mod decoders;
mod encoders;
mod processors;
mod producers;
mod resources;

use anyhow::{Context, Result};
use applications::{App, Settings};
use easy_parallel::Parallel;
use log::{error, info};
use signal_hook::{iterator::Signals, SIGINT, SIGTERM};
use smol::block_on;
use std::env;
use std::process::exit;

fn main() -> Result<()> {
    env::set_var("RUST_LOG", &Settings::get().rust_log);
    env_logger::init();

    let signals = Signals::new(&[SIGINT])?;

    Parallel::new()
        .add(|| {
            if let Err(e) = block_on(App::start()).with_context(|| "Ingestions Processor") {
                error!("{:?}", e);
                exit(1);
            }
        })
        .finish(|| {
            let code = match signals.into_iter().next() {
                Some(SIGINT) | Some(SIGTERM) => {
                    info!("Exiting on signal");
                    0
                }
                _ => {
                    error!("Unknown signal");
                    1
                }
            };

            exit(code);
        });

    #[allow(unreachable_code)]
    Ok(())
}
