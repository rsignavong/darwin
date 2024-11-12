use super::services::status;
use crate::infra::tesla::executor::TeslaExecutor;
use actix::prelude::Addr;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use listenfd::ListenFd;

pub struct State {
    pub tesla: Addr<TeslaExecutor>,
}

pub fn start() {
    let state = Data::new(State {
        tesla: TeslaExecutor::init(),
    });
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .wrap(Logger::default())
            .service(status::init())
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:5050").unwrap()
    };

    server.start();
}
