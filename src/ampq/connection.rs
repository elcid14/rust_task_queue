



use deadpool_lapin::{Pool, PoolError, Config, Runtime};
use futures_util::StreamExt;
use lapin::{
    options::*,
    types::FieldTable,
    Channel,
};
use once_cell::sync::Lazy;
use tokio::task;
use crate::task_registry::{get_task_handler};




// Define the connection pool for RabbitMQ
static RABBIT_POOL: Lazy<Pool> = Lazy::new(|| {
    let mut cfg: Config = Config::default();
    cfg.url = Some("amqp://admin:admin@localhost:5672".to_string());
    cfg.create_pool(Some(Runtime::Tokio1)).expect("Failed to create RabbitMQ pool")
});



// This function retrieves a RabbitMQ channel from the pool RABBIT_POOL
pub async fn get_rabbitmq_channel() -> Result<Channel, PoolError> {
    let conn = RABBIT_POOL.get().await?;
    let channel: Channel = conn.create_channel().await?;
    print!("Channel created");
    Ok(channel)
}


pub async fn setup_messaging(queue_names: &[&str]) -> Result<(), lapin::Error> {
    // Get a channel from the connection pool
    let channel: Channel = get_rabbitmq_channel().await.expect("Failed to get RabbitMQ channel");
    //Decalre exchanges based on queue_names
    for name in queue_names {
        channel
            .exchange_declare(
                name,
                lapin::ExchangeKind::Direct,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;
    }

    //Iterate over the queues and declare them
    for queue_name in queue_names {
        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Bind queue to exchange
        channel
            .queue_bind(
                queue_name,
                queue_name, // Bind the queue to its own exchange
                queue_name,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        // declare a consumer for the queue
        let mut consumer = channel
        .basic_consume(
            queue_name,
            &format!("{}_consumer", queue_name),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
        println!("Consumer created for queue: {}", queue_name);
            let channel_clone: Channel = channel.clone();
            task::spawn(async move {
                while let Some(result) = consumer.next().await {
                    if let Ok(delivery) = result {
                        // Extract task name from message
                        let task_name = serde_json::from_slice::<serde_json::Value>(&delivery.data)
                            .ok()
                            .and_then(|v| v.get("task")?.as_str().map(|s| s.to_string()));

                        if let Some(name) = task_name {
                            if let Some(handler) = get_task_handler(&name) {
                                tokio::spawn(handler(delivery, channel_clone.clone()));
                            } else {
                                eprintln!("No handler for task '{}'", name);
                            }
                        } else {
                            eprintln!("Malformed message");
                        }
                    }
                }
            });
        }
        Ok(())
    }
