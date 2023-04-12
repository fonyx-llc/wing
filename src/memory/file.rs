use crate::common::trigger_driver::{should_trigger, TriggerMode};
use crate::memory::register::PIPORegister;

#[derive(PartialEq, Clone)]
pub enum Command {
	Read,
	Write,
}

pub struct File {
	pub byte_quantity: usize,
	pub job: Command,
	pub clock_state: bool,
	pub trigger_mode: TriggerMode,
	pub byte_buffer: PIPORegister,
	pub address_index: usize,
	change_listeners: Vec<fn(&File)>,
	output_listeners: Vec<fn(&File)>,
}

impl File {
	pub fn new(byte_quantity: usize, trigger_mode: TriggerMode) -> File {
		File {
			byte_buffer: PIPORegister::new(byte_quantity, trigger_mode.clone()),
			byte_quantity,
			trigger_mode,
			change_listeners: Vec::new(),
			output_listeners: Vec::new(),
			clock_state: false,
			address_index: 0,
			job: Command::Read,
		}
	}

	pub fn add_change_listener(&mut self, listener: fn(&File)) {
		self.change_listeners.push(listener);
	}

	pub fn add_output_listener(&mut self, listener: fn(&File)) {
		self.output_listeners.push(listener);
	}

	pub fn set_byte(&mut self, byte: u8) {
		self.byte_buffer.set_byte(byte, self.address_index);
	}

	pub fn set_address(&mut self, address: usize) {
		self.address_index = address;
	}

	pub fn set_job(&mut self, job: Command) {
		self.job = job;
	}

	pub fn set_clock_state(&mut self, clock_state: bool) -> bool {
		let changing = self.byte_buffer.set_clock_state(clock_state);
		if changing {
			self.clock_state = clock_state;
			self.trigger();
		} else {
			self.clock_state = clock_state;
		}
		changing
	}

	fn trigger(&self) {
		if self.job == Command::Write {
			for listener in self.change_listeners.iter() {
				listener(self);
			}
		} else if self.job == Command::Read {
			for listener in self.output_listeners.iter() {
				listener(self);
			}
		}
	}
}