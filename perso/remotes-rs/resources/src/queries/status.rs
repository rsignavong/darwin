use derive_new::new;
use serde::Serialize;

#[cfg_attr(feature = "backend", derive(new))]
#[derive(Serialize)]
pub struct Status {
    pub status: usize,
}
