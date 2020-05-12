use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::{fmt::Debug, collections::VecDeque};

use crate::config;
use crate::value;

#[derive(Debug, Copy, Clone)]
pub struct TaskHandle (pub usize);
#[derive(Debug, Copy, Clone)]
pub struct ChannelHandle (pub usize);

impl TaskHandle {
	pub fn new(i: usize) -> Self {
		TaskHandle(i)
	}
}

impl ChannelHandle {
	pub fn new(i: usize) -> Self {
		ChannelHandle(i)
	}
	pub fn get_instense(self, vmc: &VMContext) -> Option<Arc<TaskContext>> {
		let r= vmc.task_pool.get(self.0);
		match r {
			Some(Some(x)) => Some(x.clone()),
			_ => None,
		}
	}
}

pub struct OpStack {
	pub stack: [value::Any; config::STACK_MAX_LENGTH],
	pub top_ptr: Cell<usize>,
}

impl OpStack {
	fn new() -> Self {
		OpStack {
			stack: [0; 256],
			top_ptr: Cell::new(0),
		}
	}
}

impl Debug for OpStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = &self.stack[0..self.top_ptr.get()]
			.iter()
			.map(u64::to_string)
			.collect::<Vec<String>>()
			.join(", ");
		write!(f, "[{}]", s)
    }
}

#[derive(Debug)]
pub struct TaskContext {
	pub task_handle: TaskHandle,
	pub channel_handle: ChannelHandle,
	pub stack: OpStack,
}

#[derive(Debug)]
pub struct VMContext {
	pub task_pool: Vec<Option<Arc<TaskContext>>>,
	pub channel_pool: Vec<Option<Arc<(Sender<value::Any>, Receiver<value::Any>)>>>,
}

unsafe impl Send for VMContext {}
unsafe impl Sync for VMContext {}

impl VMContext {
	pub fn new() -> Self {
		VMContext {
			task_pool: vec![],
			channel_pool: vec![]
		}
	}
	
	pub fn new_channel(&mut self) -> ChannelHandle {
		let handle = ChannelHandle(self.channel_pool.len());
		let r: (Sender<value::Any>, Receiver<value::Any>) = channel::<value::Any>();
		self.channel_pool.push(Some(Arc::from(r)));
		handle
	}
	
	pub fn new_task(&mut self) -> TaskHandle {
		let task_handle = TaskHandle(self.task_pool.len());
		let channel_handle = self.new_channel();
		let r = TaskContext {
			task_handle,
			channel_handle,
			stack: OpStack::new()
		};
		self.task_pool.push(Some(Arc::from(r)));
		task_handle
	}

	pub fn get_task(&self, task_handle: TaskHandle) -> Option<Arc<TaskContext>> {
		let r= self
			.task_pool
			.get(task_handle.0);
		match r {
			Some(Some(x)) => Some(x.clone()),
			_ => None,
		}
	}
	pub fn get_channek(&self, channel_handle: ChannelHandle)
		-> Option<Arc<(Sender<value::Any>, Receiver<value::Any>)>> {
		let r= self
			.channel_pool
			.get(channel_handle.0);
		match r {
			Some(Some(x)) => Some(x.clone()),
			_ => None,
		}
	}
}
