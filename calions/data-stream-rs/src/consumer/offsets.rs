use crate::common::TopicPartition;
use crate::stream::Offset;
use ahash::AHashMap;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

pub struct ConsumerOffsets(AHashMap<TopicPartition, Offset>);

impl ConsumerOffsets {
    pub fn get(&self, tp: &TopicPartition) -> Option<&Offset> {
        self.0.get(tp)
    }

    pub fn load() -> &'static Mutex<ConsumerOffsets> {
        static OFFSETS: Lazy<Mutex<ConsumerOffsets>> =
            Lazy::new(|| Mutex::new(ConsumerOffsets(AHashMap::new())));

        &OFFSETS
    }

    pub fn set(&mut self, tp: TopicPartition, offset: Offset) {
        if let Some(o) = self.get(&tp) {
            if offset > *o {
                self.0.insert(tp, offset);
            }
        }
    }
}
