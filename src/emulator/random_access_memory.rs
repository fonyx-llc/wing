use num_traits::{Num, ToPrimitive, Zero};
use super::register::{RegisterParallelInParallelOut};

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