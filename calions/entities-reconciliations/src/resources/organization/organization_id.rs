use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deref, From, Deserialize, Display, Serialize)]
pub struct OrganizationId(Uuid);
