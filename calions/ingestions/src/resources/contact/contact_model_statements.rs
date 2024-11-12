use derive_new::new;
use tokio_postgres::Statement;

#[derive(new)]
pub struct ContactModelStatements {
    delete: Statement,
    fetch: Statement,
    update: Statement,
}

impl ContactModelStatements {
    pub fn delete(&self) -> &Statement {
        &self.delete
    }

    pub fn fetch(&self) -> &Statement {
        &self.fetch
    }

    pub fn update(&self) -> &Statement {
        &self.update
    }
}
