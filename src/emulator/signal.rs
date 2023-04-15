use num_traits::{Num, ToPrimitive, Zero};

#[derive(Debug, Clone)]
pub struct Multiplexer<Selector: Copy + Num + Zero + ToPrimitive, DataWord: Copy + Num + Zero> {
	inputs: Vec<DataWord>,
	input_selector: Selector,
	chip_select: bool,
	architecture_bypass: bool,
}

impl<Selector: Copy + Num + Zero + ToPrimitive, DataWord: Copy + Num + Zero> Multiplexer<Selector, DataWord> {
	pub fn new(selection_size: Selector) -> Self {
		let architecture_bypass = selection_size.to_usize().unwrap() < 8;

		Multiplexer {
			inputs: vec![DataWord::zero(); selection_size.to_usize().unwrap()],
			input_selector: Selector::zero(),
			chip_select: false,
			architecture_bypass,
		}
	}

	pub fn set_input_word(&mut self, input_word: DataWord, input_index: Selector) {
		if input_index.to_usize().unwrap() >= self.inputs.len() { return; }
		self.inputs[input_index.to_usize().unwrap()] = input_word;
	}

	pub fn set_chip_select(&mut self, chip_select: bool) {
		self.chip_select = chip_select;
	}

	pub fn set_selector(&mut self, selector: Selector) {
		self.input_selector = selector;
	}

	pub fn read_word(&self) -> DataWord {
		if !self.chip_select { return DataWord::zero(); }
		self.inputs[self.input_selector.to_usize().unwrap()]
	}
	
	pub fn is_architecture_bypassed(&self) -> bool {
		self.architecture_bypass
	}
}

#[derive(Debug, Clone)]
pub struct Demultiplexer<Selector: Copy + Num + Zero + ToPrimitive, DataWord: Copy + Num + Zero> {
	input: DataWord,
	output_selector: Selector,
	chip_select: bool,
	architecture_bypass: bool,
}

impl<Selector: Copy + Num + Zero + ToPrimitive, DataWord: Copy + Num + Zero> Demultiplexer<Selector, DataWord> {
	pub fn new(selection_size: Selector) -> Self {
		let architecture_bypass = selection_size.to_usize().unwrap() < 8;

		Demultiplexer {
			input: DataWord::zero(),
			output_selector: Selector::zero(),
			chip_select: false,
			architecture_bypass,
		}
	}

	pub fn set_input_word(&mut self, input_word: DataWord) {
		self.input = input_word;
	}

	pub fn set_chip_select(&mut self, chip_select: bool) {
		self.chip_select = chip_select;
	}

	pub fn set_selector(&mut self, selector: Selector) {
		self.output_selector = selector;
	}

	pub fn read_word(&self, output_index: Selector) -> DataWord {
		if !self.chip_select { return DataWord::zero(); }
		if output_index == self.output_selector {
			self.input
		} else {
			DataWord::zero()
		}
	}
	
	pub fn is_architecture_bypassed(&self) -> bool {
		self.architecture_bypass
	}
}