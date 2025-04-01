use crate::http::routes::tasks::Task;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct Queue {
    pub data: Arc<Mutex<VecDeque<Task>>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            data: Arc::new(Mutex::new(VecDeque::with_capacity(50))),
        }
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}
