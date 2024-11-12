use super::Executor;
use crate::RepositoryError;
use async_trait::async_trait;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct AdvertiserPostCreated {
    send_mail: bool,
}

impl TryFrom<Row> for AdvertiserPostCreated {
    type Error = RepositoryError;

    fn try_from(r: Row) -> Result<Self, Self::Error> {
        let send_mail = r
            .try_get::<usize, bool>(0)
            .map_err(|source| RepositoryError::AdvertiserPostCreatedTryFromSendMail { source })?;

        Ok(AdvertiserPostCreated { send_mail })
    }
}

#[derive(Deserialize, Serialize)]
pub struct AdvertiserPostDetails {
    category: String,
    type_: String,
    position: String,
    description: String,
    apply: String,
    apply_email: String,
    apply_url: String,
    location: Option<String>,
    salary: Option<String>,
}

#[derive(Deserialize, Serialize, new)]
pub struct AdvertiserPostCreatorParams {
    user_id: Uuid,
    user_email: String,
    details: AdvertiserPostDetails,
}

#[async_trait]
impl Executor for AdvertiserPostCreated {}
