use derive_more::{Deref, From};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deref, Deserialize, From, Serialize, new)]
pub struct JobDetailLocation(String);
