use std::convert::Infallible;

use amqprs::connection::OpenConnectionArguments;
use deadpool::Runtime;

use crate::{Manager, Pool, PoolBuilder, PoolConfig};

/// Configuration object.
///
/// # Example
///
/// ```rs
/// use deadpool_amqprs::Config;
/// use amqprs::connection::OpenConnectionArguments;
///
/// let config = Config::new_with_con_args(OpenConnectionArguments::default())
///
/// let pool = config.create_pool();
///
/// // Do things with `pool`.
/// ```
#[derive(Clone, Default)]
pub struct Config {
    /// The [`OpenConnectionArguments`] passed to [`amqprs::connection::Connection::open`].
    pub con_args: OpenConnectionArguments,
    /// The [`PoolConfig`] passed to deadpool.
    pub pool_config: Option<PoolConfig>,
}

impl Config {
    /// Creates a new config with [`OpenConnectionArguments`] and optionally [`PoolConfig`].
    #[must_use]
    pub const fn new(con_args: OpenConnectionArguments, pool_config: Option<PoolConfig>) -> Self {
        Self {
            con_args,
            pool_config,
        }
    }

    /// Creates a new config with only [`OpenConnectionArguments`]
    #[must_use]
    pub const fn new_with_con_args(con_args: OpenConnectionArguments) -> Self {
        Self {
            con_args,
            pool_config: None,
        }
    }

    /// Creates a new pool with the current config.
    ///
    /// # Info
    ///
    /// Unlike other `deadpool-*` libs, `deadpool-amqprs` does not require user to pass [`deadpool::Runtime`],
    /// because amqprs is built on top of `tokio`, meaning one can only use `tokio` with it.
    #[must_use]
    pub fn create_pool(&self) -> Pool {
        self.builder()
            .build()
            .expect("`PoolBuilder::build` errored when it shouldn't")
    }

    /// Returns a [`PoolBuilder`] using the current config.
    ///
    /// # Info
    ///
    /// Unlike other `deadpool-*` libs, `deadpool-amqprs` does not require user to pass [`deadpool::Runtime`],
    /// because amqprs is built on top of `tokio`, meaning one can only use `tokio` with it.
    pub fn builder(&self) -> PoolBuilder {
        Pool::builder(Manager::new(self.con_args.clone()))
            .config(self.pool_config.unwrap_or_default())
            .runtime(Runtime::Tokio1)
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("pool_config", &self.pool_config)
            .finish_non_exhaustive()
    }
}

pub type ConfigError = Infallible;
