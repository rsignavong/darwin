use derive_new::new;
use tokio_postgres::Statement;

#[derive(new)]
pub struct GdprKeyModelStatements {
    fetch: Statement,
}

impl GdprKeyModelStatements {
    pub fn fetch(&self) -> &Statement {
        &self.fetch
    }
}
