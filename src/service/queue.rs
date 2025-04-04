use std::{
    collections::VecDeque,
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::http::routes::tasks::Task;

// TODO: Look up new type pattern
#[derive(Clone, Debug)]
pub struct SharedQueue<T>(Arc<Mutex<VecDeque<T>>>);

impl<T> SharedQueue<T> {
    pub fn new() -> Self {
        Self(Arc::new(Default::default()))
    }
}

impl<T> Default for SharedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Look up Deref
impl<T> Deref for SharedQueue<T> {
    type Target = Mutex<VecDeque<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SharedQueue<Task> {
    pub fn push_front(&self, task: Task) {
        let mut queue = self.lock().unwrap();
        queue.push_front(task);
    }

    pub fn push_back(&self, task: Task) {
        let mut queue = self.lock().unwrap();
        queue.push_back(task);
    }

    pub fn pop_back(&self) -> Task {
        let mut queue = self.lock().unwrap();
        queue.pop_back().unwrap()
    }
}
