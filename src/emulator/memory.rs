use std::cmp::{max, min};
use num_traits::{Zero, Num, ToPrimitive, FromPrimitive};
use crate::emulator::signal::{Demultiplexer, Multiplexer};

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
pub struct StaticRandomAccessMemory<Word: Copy + Num + Zero, Address: Copy + Num + Zero + ToPrimitive> {
	/// This demultiplexer will be used for feeding data into the demultiplexer,
	/// this demultiplexer is a wide demultiplexer that feeds data in with 1 clock cycle.
	data_feed_demultiplexer: Demultiplexer<Address, Word>,

	/// This multiplexer will be used for reading data from the memory,
	/// this multiplexer is a wide multiplexer that reads data out with 1 clock cycle.
	data_read_multiplexer: Multiplexer<Address, Word>,

	/// This demultiplexer will be used for setting whether the memory cell should read or write.
	/// This demultiplexer is a wide demultiplexer that has 2 bits of data, bit 0 for read and bit 1 for write.
	/// Bit 0 directly translates to chip_select and bit 1 directly translates to write_enable.
	///
	/// ```
	/// 0 0 0 0 0 0 0 0 [00000000] <-- u8 binary byte (word)
	///           | | |
	///           | | ^----- bit 0 (chip_select)
	///           | ^------- bit 1 (write_enable)
	///           ^--------- bit 2 (clock_bit)
	/// ```
	control_demultiplexer: Demultiplexer<Address, u8>,

	clock_bit: bool,
	queued_input: Word,
	chip_select: bool,
	write_enable: bool,
	queued_address: Address,
	memory_lines: Vec<RegisterParallelInParallelOut<Word>>,
}

impl<Word: Copy + Num + Zero, Address: Copy + Num + Zero + ToPrimitive> StaticRandomAccessMemory<Word, Address> {
	pub fn new(address_space_count: Address) -> Self {
		StaticRandomAccessMemory {
			data_feed_demultiplexer: Demultiplexer::new(address_space_count),
			data_read_multiplexer: Multiplexer::new(address_space_count),
			control_demultiplexer: Demultiplexer::new(address_space_count),
			clock_bit: false,
			queued_input: Word::zero(),
			chip_select: false,
			write_enable: false,
			queued_address: Address::zero(),
			memory_lines: vec![RegisterParallelInParallelOut::new(); address_space_count.to_usize().unwrap()],
		}
	}

	pub fn set_chip_select(&mut self, chip_select: bool) {
		self.chip_select = chip_select;
		self.data_read_multiplexer.set_chip_select(chip_select);
		self.apply_control_demultiplexer();
	}

	pub fn set_write_enable(&mut self, write_enable: bool) {
		self.write_enable = write_enable;
		self.apply_control_demultiplexer();
	}

	pub fn set_clock(&mut self, clock_bit: bool) {
		if clock_bit != self.clock_bit {
			self.clock_bit = clock_bit;
			self.apply_control_demultiplexer();
		}
	}

	pub fn set_address(&mut self, address: Address) {
		self.queued_address = address;
		self.apply_control_demultiplexer();
	}

	pub fn set_feed(&mut self, input_data: Word) {
		self.queued_input = input_data;
		self.apply_data_demultiplexer();
	}

	fn apply_control_demultiplexer(&mut self) {
		self.data_feed_demultiplexer.set_chip_select(true);
		self.control_demultiplexer.set_chip_select(true);

		let control_word = (self.chip_select as u8) | ((self.write_enable as u8) << 1) | ((self.clock_bit as u8) << 2);
		self.control_demultiplexer.set_input_word(control_word);
		self.control_demultiplexer.set_selector(self.queued_address);

		let mut address_index = Address::zero();
		for (_, memory_line) in self.memory_lines.iter_mut().enumerate() {
			let control_word = self.control_demultiplexer.read_word(address_index);
			memory_line.set_chip_select(control_word & 0b00000001 != 0);
			memory_line.set_write_enable(control_word & 0b00000010 != 0);
			memory_line.set_clock(control_word & 0b00000100 != 0);

			address_index = address_index + Address::one();
		}
	}

	fn apply_data_demultiplexer(&mut self) {
		self.data_feed_demultiplexer.set_input_word(self.queued_input);
		self.data_feed_demultiplexer.set_selector(self.queued_address);

		let mut address_index = Address::zero();
		for (_, memory_line) in self.memory_lines.iter_mut().enumerate() {
			let data_word = self.data_feed_demultiplexer.read_word(address_index);
			memory_line.set_queued_input(data_word);

			let data_word = memory_line.read_word();
			self.data_read_multiplexer.set_input_word(data_word, address_index);

			address_index = address_index + Address::one();
		}
	}

	pub fn read_word(&mut self) -> Word {
		if !self.chip_select { return Word::zero(); }
		self.data_read_multiplexer.set_selector(self.queued_address);
		let mut memory_line_index = Address::zero();
		for (_, memory_line) in self.memory_lines.iter().enumerate() {
			self.data_read_multiplexer.set_input_word(memory_line.read_word(), memory_line_index);
			memory_line_index = memory_line_index + Address::one();
		}
		self.data_read_multiplexer.read_word()
	}

	pub fn get_memory_lines_count(&self) -> usize {
		self.memory_lines.len()
	}
}

pub struct StaticRandomAccessMemoryDriver<'a, Word: Copy + Num + Zero, Address: Copy + Num + Zero + FromPrimitive + ToPrimitive> {
	memory_blocks: &'a mut [StaticRandomAccessMemory<Word, Address>],
	bits: Address,
	address: Address,
}

impl<'a, Word: Copy + Num + Zero, Address: Copy + Num + Zero + FromPrimitive + ToPrimitive> StaticRandomAccessMemoryDriver<'a, Word, Address> {
	pub fn new(memory_blocks: &'a mut [StaticRandomAccessMemory<Word, Address>]) -> Self {
		let mut total_addresses = 0;
		for memory_block in memory_blocks.iter() {
			total_addresses += memory_block.get_memory_lines_count();
		}
		let bit_count = (f64::log2(total_addresses as f64) / f64::log2(2.0)).ceil() as usize;

		StaticRandomAccessMemoryDriver {
			memory_blocks,
			bits: Address::from_usize(bit_count).unwrap(),
			address: Address::zero(),
		}
	}

	fn determine_block_index(&self, address: Address) -> usize {
		let mut address_lines_prev = Address::zero();
		for (block_index, memory_block) in self.memory_blocks.iter().enumerate() {
			let address_lines = address_lines_prev + Address::from_usize(memory_block.get_memory_lines_count()).unwrap();
			if address.to_usize().unwrap() < address_lines.to_usize().unwrap() {
				return block_index;
			}
			address_lines_prev = address_lines;
		}
		panic!("Address out of bounds");
	}

	pub fn read_word(&mut self) -> Word {
		self.memory_blocks[self.determine_block_index(self.address)].read_word()
	}

	pub fn set_chip_select(&mut self, chip_select: bool) {
		for memory_block in self.memory_blocks.iter_mut() {
			memory_block.set_chip_select(chip_select);
		}
	}

	pub fn set_write_enable(&mut self, write_enable: bool) {
		for memory_block in self.memory_blocks.iter_mut() {
			memory_block.set_write_enable(write_enable);
		}
	}

	pub fn set_clock(&mut self, clock_bit: bool) {
		for memory_block in self.memory_blocks.iter_mut() {
			memory_block.set_clock(clock_bit);
		}
	}

	pub fn set_address(&mut self, address: Address) {
		self.address = address;
		for memory_block in self.memory_blocks.iter_mut() {
			memory_block.set_address(address);
		}
	}

	pub fn set_feed(&mut self, input_data: Word) {
		for memory_block in self.memory_blocks.iter_mut() {
			memory_block.set_feed(input_data);
		}
	}

	pub fn get_address_bits(&self) -> Address {
		self.bits
	}
}

#[derive(PartialEq, Clone)]
pub enum MemoryControllerQueueJob {
	Idle,
	Read,
	Write,
}

#[derive(Clone)]
pub struct MemoryControllerQueue<Word: Copy + Num + Zero, Address: Copy + Num + Zero> {
	chip_select: bool,
	write_enable: bool,
	address: Address,
	data: Word,
	response_data: Word,
}

impl<Word: Copy + Num + Zero, Address: Copy + Num + Zero> MemoryControllerQueue<Word, Address> {
	pub fn new() -> Self {
		MemoryControllerQueue {
			chip_select: false,
			write_enable: false,
			address: Address::zero(),
			data: Word::zero(),
			response_data: Word::zero(),
		}
	}

	pub fn set_chip_select(&mut self, chip_select: bool) {
		self.chip_select = chip_select;
	}

	pub fn set_write_enable(&mut self, write_enable: bool) {
		self.write_enable = write_enable;
	}

	pub fn get_chip_select(&self) -> bool {
		self.chip_select
	}

	pub fn get_write_enable(&self) -> bool {
		self.write_enable
	}

	pub fn set_address(&mut self, address: Address) {
		self.address = address;
	}

	pub fn set_data(&mut self, data: Word) {
		self.data = data;
	}

	pub fn get_address(&self) -> Address {
		self.address
	}

	pub fn get_data(&self) -> Word {
		self.data
	}

	pub fn get_response_data(&self) -> Word {
		self.response_data
	}

	pub fn set_response_data(&mut self, response_data: Word) {
		self.response_data = response_data;
	}
}

pub struct MemoryController<'a, Word: Copy + Num + Zero, Address: Copy + Num + Zero + FromPrimitive + ToPrimitive> {
	memory_driver: &'a mut StaticRandomAccessMemoryDriver<'a, Word, Address>,
	focus: usize,
	queues: Vec<MemoryControllerQueue<Word, Address>>,
	clock: bool,
}

impl<'a, Word: Copy + Num + Zero, Address: Copy + Num + Zero + FromPrimitive + ToPrimitive> MemoryController<'a, Word, Address> {
	pub fn new(cores: usize, memory_driver: &'a mut StaticRandomAccessMemoryDriver<'a, Word, Address>) -> Self {
		MemoryController {
			memory_driver,
			focus: 0,
			queues: vec![MemoryControllerQueue::new(); cores],
			clock: false,
		}
	}

	pub fn set_clock(&mut self, clock: bool) {
		if clock != self.clock {
			self.clock = clock;
			self.memory_driver.set_chip_select(self.queues[self.focus].chip_select);
			self.memory_driver.set_write_enable(self.queues[self.focus].write_enable);
			self.memory_driver.set_address(self.queues[self.focus].address);
			self.memory_driver.set_feed(self.queues[self.focus].data);
			self.memory_driver.set_clock(clock);

			if clock { self.queues[self.focus].response_data = self.memory_driver.read_word(); }
			else if !clock {
				self.focus += 1;
				if self.focus >= self.queues.len() { self.focus = 0; }
			}
		}
	}

	pub fn set_chip_select(&mut self, chip_select: bool, core: usize) {
		self.queues[core].chip_select = chip_select;
	}

	pub fn set_write_enable(&mut self, write_enable: bool, core: usize) {
		self.queues[core].write_enable = write_enable;
	}

	pub fn set_address(&mut self, address: Address, core: usize) {
		self.queues[core].address = address;
	}

	pub fn set_feed(&mut self, input_data: Word, core: usize) {
		self.queues[core].data = input_data;
	}

	pub fn read_word(&mut self, core: usize) -> Word {
		self.queues[core].response_data
	}

	pub fn get_address_bits(&self) -> Address {
		self.memory_driver.get_address_bits()
	}
}