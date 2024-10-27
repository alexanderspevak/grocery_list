pub mod models;
use ::config::ConfigError;
use deadpool_postgres::Pool;
use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub pg: deadpool_postgres::Config,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}

pub async fn make_db_pool() -> Pool<NoTls> {
    let config = DbConfig::from_env().unwrap();
    config.pg.create_pool(NoTls).unwrap()
}
