use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use crate::routes::tasks::Task;

pub struct Queue {
    data: LazyLock<Mutex<VecDeque<Task>>>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            data: LazyLock::new(|| Mutex::new(VecDeque::with_capacity(50))),
        }
    }
}
