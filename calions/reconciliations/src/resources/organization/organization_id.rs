use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deref, Eq, From, Deserialize, Display, PartialEq, Serialize)]
pub struct OrganizationId(Uuid);
