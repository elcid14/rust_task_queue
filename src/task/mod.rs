use serde::{Serialize, Deserialize};
use serde_json::Value;







#[derive(Clone,Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}


#[derive(Clone,Serialize, Deserialize, Debug)]
pub struct Task{
    pub id: uuid::Uuid,
    pub payload: Value,
    pub retries: i32,
    pub max_retries: i32,
    pub task_status: TaskStatus

}






impl Task {
   pub fn new(payload: Vec<u8>) -> Result<Self, serde_json::Error> {
        match serde_json::from_slice::<Value>(&payload) {
            Ok(payload) => Ok(Task {
                id: uuid::Uuid::new_v4(),
                payload,
                retries: 0,
                max_retries: 5,
                task_status: TaskStatus::InProgress,
            }),
            Err(err) => {
                eprintln!("Failed to deserialize payload: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retry(&mut self) {
        if self.retries < self.max_retries {
            self.retries += 1;
            self.task_status = TaskStatus::InProgress;
        } else {
            self.task_status = TaskStatus::Failed;
        }
    }

   pub  fn complete(&mut self) {
        self.task_status = TaskStatus::Completed;
    }

   pub fn fail(&mut self) {
        self.task_status = TaskStatus::Failed;
    }


}
