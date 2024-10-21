use deadpool_postgres::Client;
use tokio_postgres::NoTls;

pub mod user;

pub trait PgDb {
    type Output;
    async fn get_by_id(
        id: &uuid::Uuid,
        client: &Client<NoTls>,
    ) -> Result<Option<Self::Output>, tokio_postgres::Error>;
    async fn insert(&self, client: &Client<NoTls>) -> Result<(), tokio_postgres::Error>;
}
