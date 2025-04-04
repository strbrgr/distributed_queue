use crate::http::routes::tasks::Task;

use super::queue::SharedQueue;

pub fn pull_from_queue(shared_queue: &mut SharedQueue<Task>) {
    let task = shared_queue.pop_back();

    println!("Task from pulled queue: {:?}", task);
}
