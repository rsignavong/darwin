use derive_more::Sub;
use derive_new::new;

#[derive(Clone, Copy, Debug, Sub, new)]
pub struct AggregateCount(#[new(value = "1")] u32);

impl AggregateCount {
    pub fn inc(&mut self) {
        self.0 += 1;
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
