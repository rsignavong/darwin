use super::Crud;
use crate::schema::tags;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct Tag {
    id: Uuid,
    tag: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "tags"]
pub struct TagCommand {
    tag: String,
}

impl Crud<TagCommand> for Tag {
    fn read(conn: &PgConn, tag_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::tags::dsl::*;
        Ok(tags.find(tag_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, tag_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::tags::dsl::*;
        Ok(diesel::delete(tags.find(tag_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &TagCommand) -> Result<Self, RepositoryError> {
        use crate::schema::tags::dsl::*;
        let tag_ = diesel::insert_into(tags)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(tag_)
    }

    fn update(conn: &PgConn, tag_id: &Uuid, cmd: &TagCommand) -> Result<Self, RepositoryError> {
        use crate::schema::tags::dsl::*;
        let tag_ = diesel::update(tags.find(tag_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(tag_)
    }
}
