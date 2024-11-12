use crate::applications::{Settings, TeslaConfig};
use crate::core::Tesla;
use actix::prelude::{Actor, Addr};
use actix::sync::{SyncArbiter, SyncContext};
use std::sync::Arc;

pub struct TeslaExecutor(Arc<Tesla>);

impl Actor for TeslaExecutor {
    type Context = SyncContext<Self>;
}

impl TeslaExecutor {
    pub fn init() -> Addr<TeslaExecutor> {
        let TeslaConfig {
            data,
            file_size,
            pool_size,
        } = &Settings::get().tesla;
        let tesla = Arc::new(Tesla::init(&data, *file_size).expect("Failed to initialize Tesla"));

        SyncArbiter::start(*pool_size as usize, move || TeslaExecutor(tesla.clone()))
    }

    pub fn tesla(&self) -> &Tesla {
        &self.0
    }
}
