use crate::common::trigger_driver::{should_trigger, TriggerMode};

pub struct PIPORegister {
	pub byte_quantity: usize,
	pub clock_state: bool,
	pub trigger_mode: TriggerMode,
	pub byte_buffer: Vec<u8>,
	change_listeners: Vec<fn(&PIPORegister)>,
}

impl PIPORegister {
	pub fn new(byte_quantity: usize, trigger_mode: TriggerMode) -> PIPORegister {
		PIPORegister {
			clock_state: false,
			change_listeners: Vec::new(),
			byte_buffer: vec![0; byte_quantity],
			byte_quantity,
			trigger_mode,
		}
	}

	pub fn add_change_listener(&mut self, listener: fn(&PIPORegister)) {
		self.change_listeners.push(listener);
	}

	pub fn set_byte(&mut self, byte: u8, index: usize) {
		self.byte_buffer[index] = byte;
	}

	pub fn set_clock_state(&mut self, clock_state: bool) -> bool {
		let changing = should_trigger(self.clock_state, clock_state, self.trigger_mode.clone());
		if changing {
			self.clock_state = clock_state;
			self.trigger();
		} else {
			self.clock_state = clock_state;
		}
		changing
	}

	fn trigger(&self) {
		for listener in self.change_listeners.iter() {
			listener(self);
		}
	}
}