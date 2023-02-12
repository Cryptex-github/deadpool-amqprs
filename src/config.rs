use amqprs::connection::OpenConnectionArguments;
use deadpool::managed::{reexports::PoolConfig, PoolBuilder};

#[derive(Default, Clone)]
pub struct Config {
    pub con_args: OpenConnectionArguments,
    pub pool_config: Option<PoolConfig>
}
