use task_macro::task;
use serde_json::Value;
use crate::task::Task;



#[task(name = "example_task")]
async fn example_task(payload: Value, task: Task) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Running task: {}", task.id);
    println!("Payload: {:?}", payload);
    Ok(())
}
