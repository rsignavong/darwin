mod assets;
mod config;
mod cookie;
mod error;
mod handlers;
mod router;
mod templates;

pub use config::WebConfig;
pub use cookie::Cookie;
pub use error::WebError;
pub use templates::Templates;

use axum::Server;
use router::Router;
use services::Services;
use std::{net::SocketAddr, str::FromStr};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;

pub struct Web;

impl Web {
    pub fn start(
        listen: &'static str,
        cookie: &Cookie,
        services: Services,
    ) -> Result<(), WebError> {
        // TODO
        // cookie
        // service extension
        let addr = SocketAddr::from_str(listen);
        Server::bind(&addr)
            .serve(Router::new(services))
            .with_graceful_shutdown(Web::shutdown_signal())
            .await?;

        info!("Listening on {}", addr);

        Ok(())
    }

    #[cfg(unix)]
    async fn shutdown_signal() {
        use std::io;
        use tokio::signal::unix::SignalKind;

        async fn terminate() -> io::Result<()> {
            tokio::signal::unix::signal(SignalKind::terminate())?
                .recv()
                .await;
            Ok(())
        }

        tokio::select! {
            _ = terminate() => {},
            _ = tokio::signal::ctrl_c() => {}
        }

        info!("Signal received, shutdowning...");
    }

    #[cfg(windows)]
    async fn shutdown_signal() {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
        info!("Signal received, shutdowning...");
    }
}
