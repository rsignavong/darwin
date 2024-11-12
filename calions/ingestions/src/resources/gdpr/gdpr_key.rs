use super::{GdprKeyModel, GdprKeyType};
use crate::resources::OrganizationId;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct GdprKey {
    pub type_: GdprKeyType,
    pub key: Arc<GdprKeyModel>,
    pub organization_id: Arc<OrganizationId>,
}
