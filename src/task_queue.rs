use task_macro::task;
use serde_json::Value;
// Ensure the correct path to Task is imported or define Task if missing
// Adjusted import path to resolve the error
use crate::task::Task;



#[task(name = "example_task")]
async fn example_task(payload: Value, task: Task) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Running task: {}", task.id);
    println!("Payload: {:?}", payload);
    Ok(())
}
