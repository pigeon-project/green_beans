#[macro_use]
extern crate lazy_static;
extern crate corn;
extern crate async_std;

mod config;
mod value;
mod context;
mod execenv;

use std::{cell::Ref, sync::Mutex};
use async_std::task;
use async_std::prelude::*;

use context::*;

lazy_static! {
    pub static ref GLOBAL_CONTEXT: Mutex<VMContext> = Mutex::new(VMContext::new());
}

#[async_std::main]
async fn main() {
    // let gc: Ref<Mutex<VMContext>> = GLOBAL_CONTEXT;
    let mut v = vec![];
    for _ in 0..=100 {
        v.push(task::spawn(async {
            GLOBAL_CONTEXT.lock().unwrap().new_task()
        }));
    }

    for i in v {
        println!("task: {:?}", i.await);
    }
    
    println!("Hello, world!");
}
