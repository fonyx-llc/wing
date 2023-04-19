//! FF (Flip Flop) module contains logical FF emulators which perform realistic operations
//! of which would be done by a real FF integrated circuit.

/// SRFF (Set Reset Flip Flop) is a clocked FF (Flip Flop) with 2 inputs.
/// This FF allows for basic bit or word setting with reset
/// functionality.
pub struct SrFf {
	pub bit: bool,
	pub clock: bool,

	/// If this is true, the flip flop will trigger on the falling edge of the clock,
	/// otherwise it will trigger on the rising edge.
	pub trigger_falling: bool,

	/// This is the input of the flip flip that is queued to be applied when the clock
	/// meets the trigger mode.
	pub input_queue: bool,

	/// Reset input queue which is used to keep track of the alternate input.
	pub reset_queue: bool,
}

impl SrFf {
	/// # Arguments
	///  - trigger_mode: When true, the flip flop will trigger on the falling edge of the clock,
	/// otherwise it will trigger on the rising edge.
	pub fn new(trigger_falling: bool) -> Self {
		Self {
			bit: false,
			clock: false,
			trigger_falling,
			input_queue: false,
			reset_queue: false,
		}
	}

	pub fn set(&mut self, input_bit: bool) {
		self.input_queue = input_bit;
	}

	pub fn reset(&mut self, input_bit: bool) {
		self.reset_queue = input_bit;
	}

	pub fn set_clock(&mut self, clock_bit: bool) {
		if self.clock != clock_bit {
			self.clock = clock_bit;
			if (self.trigger_falling && !self.clock) || (!self.trigger_falling && self.clock) {
				if self.reset_queue && !self.input_queue {
					self.bit = false;
				} else if !self.reset_queue && self.input_queue {
					self.bit = true;
				} else if self.reset_queue && self.input_queue {
					self.bit = !self.bit;
				}
			}
		}
	}
}

#[test]
fn test_sr_ff_rising() {
	let mut sr_ff = SrFf::new(false);

	// Test basic set control
	sr_ff.set(true);
	sr_ff.set_clock(false);
	assert!(!sr_ff.bit);
	sr_ff.set_clock(true);
	assert!(sr_ff.bit);

	// Test toggle logic
	sr_ff.reset(true);
	sr_ff.set_clock(false);
	assert!(sr_ff.bit);
	sr_ff.set_clock(true);
	assert!(!sr_ff.bit);
	sr_ff.set_clock(false);

	// Test reset control
	sr_ff.set(false);
	sr_ff.set_clock(true);
	assert!(!sr_ff.bit);
	sr_ff.set_clock(false);
	assert!(!sr_ff.bit);
}

#[test]
fn test_sr_ff_falling() {
	let mut sr_ff = SrFf::new(true);

	// Test basic set control
	sr_ff.set(true);
	sr_ff.set_clock(true);
	assert!(!sr_ff.bit);
	sr_ff.set_clock(false);
	assert!(sr_ff.bit);

	// Test toggle logic
	sr_ff.reset(true);
	sr_ff.set_clock(true);
	assert!(sr_ff.bit);
	sr_ff.set_clock(false);
	assert!(!sr_ff.bit);
	sr_ff.set_clock(true);

	// Test reset control
	sr_ff.set(false);
	sr_ff.set_clock(false);
	assert!(!sr_ff.bit);
	sr_ff.set_clock(true);
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
			trigger_falling,
			input_queue: false,
		}
	}

	pub fn set_input(&mut self, input_bit: bool) {
		self.input_queue = input_bit;
	}

	pub fn set_clock(&mut self, clock_bit: bool) {
		if self.clock != clock_bit {
			self.clock = clock_bit;
			if ((self.trigger_falling && !self.clock) || (!self.trigger_falling && self.clock))
				&& self.input_queue { self.bit = !self.bit; }
		}
	}
}

#[test]
fn test_t_ff_rising() {
	let mut t_ff = TFf::new(false);

	// Test basic toggle from false
	assert!(!t_ff.bit);
	t_ff.set_input(true);
	t_ff.set_clock(false);
	assert!(!t_ff.bit);
	t_ff.set_clock(true);
	assert!(t_ff.bit);
	t_ff.set_clock(false);

	// Test basic toggle from true
	assert!(t_ff.bit);
	t_ff.set_clock(true);
	assert!(!t_ff.bit);
	t_ff.set_clock(false);

	// Ensure false input does not change state
	t_ff.set_input(false);
	assert!(!t_ff.bit);
	t_ff.set_clock(true);
	assert!(!t_ff.bit);
	t_ff.set_clock(false);

	// Ensure dual clock duplicate state does not change state
	t_ff.set_input(true);
	assert!(!t_ff.bit);
	t_ff.set_clock(true);
	assert!(t_ff.bit);
	t_ff.set_clock(true);
	assert!(t_ff.bit);
	t_ff.set_clock(false);
}

#[test]
fn test_t_ff_falling() {
	let mut t_ff = TFf::new(true);

	// Test basic toggle from false
	assert!(!t_ff.bit);
	t_ff.set_input(true);
	t_ff.set_clock(true);
	assert!(!t_ff.bit);
	t_ff.set_clock(false);
	assert!(t_ff.bit);
	t_ff.set_clock(true);

	// Test basic toggle from true
	assert!(t_ff.bit);
	t_ff.set_clock(false);
	assert!(!t_ff.bit);
	t_ff.set_clock(true);

	// Ensure false input does not change state
	t_ff.set_input(false);
	assert!(!t_ff.bit);
	t_ff.set_clock(false);
	assert!(!t_ff.bit);
	t_ff.set_clock(true);

	// Ensure dual clock duplicate state does not change state
	t_ff.set_input(true);
	assert!(!t_ff.bit);
	t_ff.set_clock(false);
	assert!(t_ff.bit);
	t_ff.set_clock(false);
	assert!(t_ff.bit);
	t_ff.set_clock(true);
}