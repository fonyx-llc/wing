use crate::latch::WordNumber;
/// SRLatch (Set Reset) is a latch with 2 inputs and outputs.
/// The latch allows for basic bit or word setting with reset
/// functionality.
pub struct SRLatch<Word: WordNumber> {
	value: Word,
}

impl<Word: WordNumber> SRLatch<Word> {
	pub fn new() -> Self {
		Self {
			value: Word::zero(),
		}
	}

	pub fn set(&mut self, value: Word) {
		self.value = value;
	}

	pub fn reset(&mut self) {
		self.value = Word::zero();
	}
}

impl<Word: WordNumber> Default for SRLatch<Word> {
	fn default() -> Self {
		Self::new()
	}
}