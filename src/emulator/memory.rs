use num_traits::{Zero, Num, ToPrimitive};

#[derive(Debug, Clone)]
pub struct RegisterParallelInParallelOut<Word: Copy + Num + Zero> {
	clock_bit: bool,
	queued_input: Word,
	chip_select: bool,
	write_enable: bool,
	data_word: Word,
}

impl<Word: Copy + Num + Zero> RegisterParallelInParallelOut<Word> {
	pub fn new() -> Self {
		RegisterParallelInParallelOut {
			clock_bit: false,
			queued_input: Word::zero(),
			data_word: Word::zero(),
			chip_select: false,
			write_enable: false,
		}
	}

	pub fn set_clock(&mut self, clock_bit: bool) {
		if !self.write_enable { return; }
		if clock_bit != self.clock_bit {
			self.clock_bit = clock_bit;
			if clock_bit {
				self.data_word = self.queued_input;
			}
		}
	}

	pub fn set_queued_input(&mut self, input_data: Word) {
		self.queued_input = input_data;
	}

	pub fn set_chip_select(&mut self, chip_select: bool) {
		self.chip_select = chip_select;
	}

	pub fn set_write_enable(&mut self, chip_enable: bool) {
		self.write_enable = chip_enable;
	}

	pub fn read_word(&self) -> Word {
		if !self.chip_select { return Word::zero(); }
		self.data_word
	}
}

impl Default for RegisterParallelInParallelOut<u8> {
	fn default() -> Self {
		RegisterParallelInParallelOut::new()
	}
}

#[derive(Debug, Clone)]
pub struct RandomAccessMemory<Word: Copy + Num + Zero, Address: Copy + Num + Zero + ToPrimitive> {
	content: Vec<RegisterParallelInParallelOut<Word>>,
	clock_bit: bool,
	queued_input: Word,
	chip_select: bool,
	write_enable: bool,
	queued_address: Address,
}

impl<Word: Copy + Num + Zero, Address: Copy + Num + Zero + ToPrimitive> RandomAccessMemory<Word, Address> {
	pub fn new(address_count: Address) -> RandomAccessMemory<Word, Address> {
		RandomAccessMemory {
			content: vec![RegisterParallelInParallelOut::new(); address_count.to_usize().unwrap()],
			clock_bit: false,
			queued_input: Word::zero(),
			chip_select: false,
			write_enable: false,
			queued_address: Address::zero(),
		}
	}
}