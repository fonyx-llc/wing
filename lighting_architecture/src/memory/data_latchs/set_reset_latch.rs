use num_traits::{Num, Zero};

/// A set and reset latch can be used for setting binary information and resetting it on command.
///
/// # Arguments
/// - Word: The word, which is the actual raw unformatted data being sent through.
#[derive(Debug, Clone)]
pub struct SetResetLatch<Word: Num + Zero> {
	/// This data is currently stored actively inside the module.
	pub stored_word: Word,
}

impl<Word: Num + Zero> SetResetLatch<Word> {
	/// Initialize a new Set and Reset latch.
	pub fn new() -> Self {
		SetResetLatch {
			stored_word: Word::zero()
		}
	}

	/// Set the stored word to the given word.
	///
	/// # Arguments
	/// - word: The word to set the stored word to.
	pub fn set(&mut self, word: Word) {
		self.stored_word = word;
	}

	/// Reset the stored word to zero.
	pub fn reset(&mut self) {
		self.stored_word = Word::zero();
	}
}

impl Default for SetResetLatch<u8> {
	/// Initialize a new Set and Reset latch.
	fn default() -> Self {
		SetResetLatch::new()
	}
}

#[test]
pub fn test_init_unclocked() {
	let latch = SetResetLatch::<u8>::new();
	assert_eq!(latch.stored_word, 0);
}

#[test]
pub fn test_set_unclocked() {
	let mut latch = SetResetLatch::<u8>::new();
	latch.set(0b10101010);
	assert_eq!(latch.stored_word, 0b10101010);
}

#[test]
pub fn test_reset_unclocked() {
	let mut latch = SetResetLatch::<u8>::new();
	latch.set(0b10101010);
	latch.reset();
	assert_eq!(latch.stored_word, 0);
}

#[test]
pub fn test_set_reset_then_set_unclocked() {
	let mut latch = SetResetLatch::<u8>::new();
	latch.set(0b10101010);
	latch.reset();
	latch.set(0b01010101);
	assert_eq!(latch.stored_word, 0b01010101);
}

#[test]
pub fn test_32_bits_unclocked() {
	let mut latch = SetResetLatch::<u32>::new();
	latch.set(0b10101010_01010101_10101010_01010101);
	assert_eq!(latch.stored_word, 0b10101010_01010101_10101010_01010101);
}

/// The trigger mode of the clock is used to specify a trigger watcher when an operation
/// should trigger based on the timings of a clock signal.
#[derive(PartialEq, Debug, Clone)]
pub enum ClockTriggerMode {
	/// This will trigger when the clock signal is rising.
	Rising,

	/// This will trigger when the clock signal is falling.
	Falling,

	/// This will trigger when the clock signal is rising or falling.
	RisingFalling,
}

/// Clocked set and reset latch. This latch will only perform operation when the clock signal
/// is sent.
///
/// # Arguments
/// - Word: The word, which is the actual raw unformatted data being sent through.
#[derive(Debug, Clone)]
pub struct ClockedSetResetLatch<Word: Num + Zero + Clone> {
	/// This data is currently stored actively inside the module.
	pub stored_word: Word,

	/// This is the state of the clock signal.
	pub clock_state: bool,

	/// This is the trigger mode of the clock. When set, this will allow for the save
	/// call to be trigger when the clock is either rising, falling, or both.
	pub clock_trigger_mode: ClockTriggerMode,

	/// This input is associated directly with the data input of the latch. Based on the
	/// applied trigger mode, the clock state, and time of state change, this will be applied
	/// to the saved latch contents.
	pub direct_input: Word,
}

impl<Word: Num + Zero + Clone> ClockedSetResetLatch<Word> {
	/// Initialize a new set and reset latch with a clock trigger system.
	///
	/// # Arguments
	/// - trigger_mode: The trigger reason or mode for the clock. Based on this, the clock state, and the time of
	/// state change, the latch will be triggered to write.
	pub fn new(trigger_mode: ClockTriggerMode) -> Self {
		ClockedSetResetLatch {
			stored_word: Word::zero(),
			clock_state: false,
			clock_trigger_mode: trigger_mode,
			direct_input: Word::zero(),
		}
	}

	/// Set the state of the clock, based on this signal and the trigger mode, it will
	/// be determined whether or not to perform an operation.
	///
	/// # Arguments
	/// - state: The state of the clock signal.
	pub fn set_clock(&mut self, state: bool) {
		if self.clock_state != state {
			self.clock_state = state;
			if self.clock_trigger_mode == ClockTriggerMode::RisingFalling
				|| state && self.clock_trigger_mode == ClockTriggerMode::Rising
				|| !state && self.clock_trigger_mode == ClockTriggerMode::Falling {
				self.stored_word = self.direct_input.clone();
			}
		}
	}

	/// Set the queued data input so that when the clock handler determines that
	/// data should be saved, this will be the data that's being saved.
	///
	/// # Arguments
	/// - word: The word to set the data input to.
	pub fn set_data_input(&mut self, word: Word) {
		self.direct_input = word;
	}

	/// Reset the stored word to zero. This will also reset the data input.
	/// This will not reset the clock state.
	pub fn reset(&mut self) {
		self.stored_word = Word::zero();
		self.direct_input = Word::zero();
	}
}

#[test]
pub fn test_init() {
	let latch = ClockedSetResetLatch::<u8>::new(ClockTriggerMode::Rising);
	assert_eq!(latch.stored_word, 0);
	assert_eq!(latch.clock_trigger_mode, ClockTriggerMode::Rising);
	assert_eq!(latch.direct_input, 0);
	assert!(!latch.clock_state);
}

#[test]
pub fn test_set_clock_rising() {
	let mut latch = ClockedSetResetLatch::<u8>::new(ClockTriggerMode::Rising);

	latch.set_data_input(0b10101010);
	latch.set_clock(true);
	assert_eq!(latch.stored_word, 0b10101010);
	assert!(latch.clock_state);

	latch.set_clock(false);
	assert_eq!(latch.stored_word, 0b10101010);
	assert!(!latch.clock_state);
}

#[test]
pub fn test_set_clock_falling() {
	let mut latch = ClockedSetResetLatch::<u8>::new(ClockTriggerMode::Falling);

	latch.set_data_input(0b10101010);
	latch.set_clock(true);
	assert_eq!(latch.stored_word, 0b00000000);
	assert!(latch.clock_state);

	latch.set_clock(false);
	assert_eq!(latch.stored_word, 0b10101010);
	assert!(!latch.clock_state);
}

#[test]
pub fn test_set_clock_rising_falling() {
	let mut latch = ClockedSetResetLatch::<u8>::new(ClockTriggerMode::RisingFalling);

	latch.set_data_input(0b10101010);
	latch.set_clock(true);
	assert_eq!(latch.stored_word, 0b10101010);
	assert!(latch.clock_state);

	latch.set_clock(false);
	assert_eq!(latch.stored_word, 0b10101010);
	assert!(!latch.clock_state);

	latch.set_clock(true);
	assert_eq!(latch.stored_word, 0b10101010);
	assert!(latch.clock_state);
}