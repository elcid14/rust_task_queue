use std::{collections::HashMap};

enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}


struct Task{
    id: String,
    payload: HashMap<String, String>,
    retries: i32,
    max_retries: i32,
    task_status: TaskStatus

}


impl Task {
    fn new(id: String, payload: HashMap<String, String>, max_retries: i32, task_status: TaskStatus) -> Self {

        return Task {
            id,
            payload,
            retries: 0,
            max_retries: 5,
            task_status: TaskStatus::InProgress
        }

    }
}






fn main() {
    let mut task = Task::new("1".to_string(), HashMap::new(), 5, TaskStatus::Pending);

    println!("Hello, world!");
}
