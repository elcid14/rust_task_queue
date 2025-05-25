// task_registry.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::pin::Pin;
use lapin::message::Delivery;
use once_cell::sync::Lazy;
use lapin::Channel;

pub type TaskFn = Box<dyn Fn() + Send + Sync + 'static>;

pub type TaskHandler = Arc<dyn Fn(Delivery, Channel) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;


static TASKS: Lazy<Mutex<HashMap<String, TaskFn>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static REGISTRY: Lazy<Mutex<HashMap<String, TaskHandler>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_task_handler(name: &str, handler: TaskHandler) {
    REGISTRY.lock().unwrap().insert(name.to_string(), handler);
}

pub fn get_task_handler(name: &str) -> Option<TaskHandler> {
    REGISTRY.lock().unwrap().get(name).cloned()
}

pub fn debug_print_handlers() {
    let map = REGISTRY.lock().unwrap();
    println!("Registered tasks:");
    for k in map.keys() {
        println!("- {}", k);
    }
}



pub fn run_task(name: &str) -> bool {
    if let Some(task) = TASKS.lock().unwrap().get(name) {
        task();
        true
    } else {
        false
    }
}
