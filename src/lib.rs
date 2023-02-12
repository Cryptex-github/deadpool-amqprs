pub mod config;

use amqprs::connection::{OpenConnectionArguments, Connection};
use deadpool::managed::{RecycleResult, RecycleError};
pub use deadpool::{async_trait, managed::{self, reexports::*}};

pub use config::Config;

pub struct Manager {
    con_args: OpenConnectionArguments
}

impl Manager {
    #[must_use]
    pub fn new(con_args: OpenConnectionArguments) -> Self {
        Self { con_args }
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Connection;
    type Error = amqprs::error::Error;

    async fn create(&self) -> Result<Connection, Self::Error> {
        Ok(Connection::open(&self.con_args).await?)
    }

    async fn recycle(&self, conn: &mut Self::Type) -> RecycleResult<Self::Error> {
        Ok(())
    }
}
