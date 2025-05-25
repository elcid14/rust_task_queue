mod ampq;
mod task_queue;
mod task_registry;
mod task;
use ampq::connection::setup_messaging;










// // sample task macro usage
// #[task(name = "example_task")]
// async fn example_task(payload: Value, task: Task) -> Result<(), Box<dyn std::error::Error>> {
//     println!("Running task: {}", task.id);
//     println!("Payload: {:?}", payload);
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
env_logger::init();
let queue_names: Vec<&'static str> = vec!["test_queue_1", "test_queue_2"];
setup_messaging(&queue_names).await.expect("Failed to setup messaging");
println!("Messaging setup complete");
// // sample task macro usage

tokio::signal::ctrl_c().await?;
println!("Shutdown signal received.");
Ok(())

}
