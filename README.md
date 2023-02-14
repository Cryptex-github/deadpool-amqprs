# deadpool-amqprs

## Deadpool for amqprs

Deadpool is a dead simple async pool for connections and objects of any type.

This crate implements a [`deadpool`](https://crates.io/crates/deadpool) manager for [`amqprs`](https://crates.io/crates/amqprs).

## Example

```rs
use deadpool_amqprs::Config;
use amqprs::{callbacks::{DefaultChannelCallback, DefaultConnectionCallback}, connection::OpenConnectionArguments};

#[tokio::main]
async fn main() {
    let config = Config::new_with_con_args(OpenConnectionArguments::default());
    let pool = config.create_pool();
    
    let con = pool.get().await.unwrap();
    con.register_callback(DefaultConnectionCallback).await.unwrap();

    let channel = con.open_channel().await.unwrap();
    channel.register_callback(DefaultChannelCallback).await.unwrap();

    // Do stuff with `channel`.
}
```
