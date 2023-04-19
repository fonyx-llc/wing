//! FF (Flip Flop) module contains logical FF emulators which perform realistic operations
//! of which would be done by a real FF integrated circuit.

/// SRFF (Set Reset Flip Flop) is a FF (Flip Flop) with 2 inputs.
/// This FF allows for basic bit or word setting with reset
/// functionality.
pub struct SrFf {
	bit: bool,
}

impl SrFf {
	pub fn new() -> Self {
		Self {
			bit: false,
		}
	}

	pub fn set(&mut self) {
		self.bit = true;
	}

	pub fn reset(&mut self) {
		self.bit = false;
	}
}

impl Default for SrFf {
	fn default() -> Self {
		Self::new()
	}
}

#[test]
fn test_sr_ff() {
	let mut sr_ff = SrFf::new();

	sr_ff.set();
	assert!(sr_ff.bit);

	sr_ff.reset();
	assert!(!sr_ff.bit);
}

/// TFF (Toggle Flip Flop) is a clocked FF (Flip Flop) with 1 input.
/// This FF will toggle its internal value if the input is high
/// and the clock meets the trigger mode.
pub struct TFf {
	pub bit: bool,
	pub clock: bool,

	/// If this is true, the flip flop will trigger on the falling edge of the clock,
	/// otherwise it will trigger on the rising edge.
	pub trigger_falling: bool,

	/// This is the input of the flip flip that is queued to be applied when the clock
	/// meets the trigger mode.
	pub input_queue: bool,
}

impl TFf {
	/// # Arguments
	///  - trigger_mode: When true, the flip flop will trigger on the falling edge of the clock,
	/// otherwise it will trigger on the rising edge.
	pub fn new(trigger_falling: bool) -> Self {
		Self {
			bit: false,
			clock: false,
			trigger_falling: false,
			input_queue: false,
		}
	}

	pub fn set_input(&mut self, input_bit: bool) {
		self.input_queue = input_bit;
	}

	pub fn set_clock(&mut self, clock_bit: bool) {
		if self.clock != clock_bit {
			self.clock = clock_bit;
			if (self.trigger_falling && !self.clock) || (!self.trigger_falling && self.clock) { self.bit = self.input_queue; }
		}
	}
}

#[test]
fn test_t_ff_rising() {
	let mut t_ff = TFf::new(false);

	t_ff.set_input(true);
	t_ff.set_clock(false);
	assert!(!t_ff.bit);

	t_ff.set_clock(true);
	assert!(t_ff.bit);

	t_ff.set_clock(false);
	assert!(t_ff.bit);

	t_ff.set_input(false);
	assert!(t_ff.bit);
	t_ff.set_clock(true);
	assert!(!t_ff.bit);
	t_ff.set_clock(true);
	assert!(!t_ff.bit);
	t_ff.set_clock(false);
	assert!(!t_ff.bit);
}

#[test]
fn test_t_ff_falling() {
	let mut t_ff = TFf::new(true);

	t_ff.set_input(true);
	t_ff.set_clock(true);
	assert!(!t_ff.bit);

	t_ff.set_clock(false);
	assert!(t_ff.bit);

	t_ff.set_clock(true);
	assert!(t_ff.bit);

	t_ff.set_input(false);
	assert!(t_ff.bit);
	t_ff.set_clock(false);
	assert!(!t_ff.bit);
	t_ff.set_clock(false);
	assert!(!t_ff.bit);
	t_ff.set_clock(true);
	assert!(!t_ff.bit);
}