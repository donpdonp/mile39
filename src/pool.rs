use std::thread::{spawn, JoinHandle};

pub struct Pool {
    pool: Vec<JoinHandle<()>>
}

pub fn new() -> Pool {
    return Pool{pool: Vec::new()}
}

impl Pool {
    pub fn len(&self) -> usize {
        return self.pool.len()
    }

    pub fn push<F>(&mut self, str: F)  where
      F: FnOnce() -> () + std::marker::Send + 'static{
        self.pool.push(spawn(str));
    }
}
