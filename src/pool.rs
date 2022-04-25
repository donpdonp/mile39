use std::thread::{spawn, JoinHandle};

pub struct Pool {
    pool: Vec<JoinHandle<()>>,
}

pub fn new() -> Pool {
    return Pool { pool: Vec::new() };
}

impl Pool {
    pub fn len(&self) -> usize {
        return self.pool.len();
    }

    pub fn push<F>(&mut self, func: F)
    where
        F: FnOnce() -> () + std::marker::Send + 'static,
    {
        self.pool.push(spawn(func));
    }
}
