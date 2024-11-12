use super::ConsumerError;
use parking_lot::RwLock;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::Arc;

pub struct ConsumerPool {
    pool: ThreadPool,
    stop: Arc<RwLock<bool>>,
}

impl ConsumerPool {
    pub fn new(nb: usize) -> Result<Self, ConsumerError> {
        let pool = ThreadPoolBuilder::new().num_threads(nb).build()?;
        let stop = Arc::new(RwLock::new(false));

        Ok(ConsumerPool { pool, stop })
    }

    pub fn spawn<OP>(&self, op: OP)
    where
        OP: FnOnce(Arc<RwLock<bool>>) + Send + 'static,
    {
        let stop = self.stop.clone();
        self.pool.spawn(move || op(stop));
    }

    pub fn stop(&self) {
        let mut stop = self.stop.write();
        *stop = true;
    }
}
