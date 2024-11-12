use super::Crud;
use crate::schema::features;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct Feature {
    id: Uuid,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "features"]
pub struct FeatureCommand {
    name: String,
    description: String,
}

impl Crud<FeatureCommand> for Feature {
    fn read(conn: &PgConn, feature_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::features::dsl::*;
        Ok(features.find(feature_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, feature_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::features::dsl::*;
        Ok(diesel::delete(features.find(feature_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &FeatureCommand) -> Result<Self, RepositoryError> {
        use crate::schema::features::dsl::*;
        let feature = diesel::insert_into(features)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(feature)
    }

    fn update(
        conn: &PgConn,
        feature_id: &Uuid,
        cmd: &FeatureCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::features::dsl::*;
        let feature = diesel::update(features.find(feature_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(feature)
    }
}
