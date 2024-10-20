#![doc = include_str!("../README.md")]
#![allow(clippy::module_name_repetitions)]

pub mod config;

pub use amqprs;
use amqprs::connection::OpenConnectionArguments;
use deadpool::managed;
pub use deadpool::managed::reexports::*;
use deadpool::managed::{RecycleError, RecycleResult};

pub use config::{Config, ConfigError};

deadpool::managed_reexports!(
    "amqprs",
    Manager,
    managed::Object<Manager>,
    amqprs::error::Error,
    ConfigError
);

struct Placeholder;

impl std::fmt::Debug for Placeholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "..")
    }
}

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
        f.debug_struct("Manager")
            .field("con_args", &Placeholder)
            .finish()
    }
}

impl managed::Manager for Manager {
    /// Type of [`Object`]s that this [`Manager`] creates and recycles.
    type Type = amqprs::connection::Connection;
    /// Error that this [`Manager`] can return when creating and/or recycling
    /// [`Object`]s.
    type Error = amqprs::error::Error;

    /// Creates a new instance of [`Manager::Type`].
    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Self::Type::open(&self.con_args).await
    }

    /// Tries to recycle an instance of [`Manager::Type`].
    ///
    /// # Errors
    ///
    /// Returns [`Manager::Error`] if the instance couldn't be recycled.
    async fn recycle(&self, conn: &mut Self::Type, _: &Metrics) -> RecycleResult<Self::Error> {
        if conn.is_open() {
            Ok(())
        } else {
            Err(RecycleError::message("Connection closed."))
        }
    }
}
