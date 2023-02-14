#![doc = include_str!("../README.md")]
#![allow(clippy::module_name_repetitions)]

pub mod config;

pub use amqprs;
use amqprs::connection::OpenConnectionArguments;
pub use deadpool::managed::reexports::*;
use deadpool::managed::{RecycleError, RecycleResult};
use deadpool::{async_trait, managed};

pub use config::{Config, ConfigError};

deadpool::managed_reexports!(
    "amqprs",
    Manager,
    managed::Object<Manager>,
    amqprs::error::Error,
    ConfigError
);

/// Type alias for [`Object`] in case Object isn't straight foward enough.
pub type Connection = Object;

/// [`Manager`] for creating and recycling [`amqprs`] connections.
pub struct Manager {
    con_args: OpenConnectionArguments,
}

impl Manager {
    /// Creates a new [`Manager`] from the given arguments.
    #[must_use]
    #[inline]
    pub const fn new(con_args: OpenConnectionArguments) -> Self {
        Self { con_args }
    }
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Manager").finish_non_exhaustive()
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Type = amqprs::connection::Connection;
    type Error = amqprs::error::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(Self::Type::open(&self.con_args).await?)
    }

    async fn recycle(&self, conn: &mut Self::Type) -> RecycleResult<Self::Error> {
        if conn.is_open() {
            Ok(())
        } else {
            Err(RecycleError::StaticMessage("Connection closed."))
        }
    }
}
