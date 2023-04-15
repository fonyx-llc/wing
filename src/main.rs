pub mod emulator;
use emulator::register::{RegisterParallelInParallelOut};
use crate::emulator::signal::{Demultiplexer, Multiplexer};

fn main() {
    let mut demux: Demultiplexer<u8, u8> = Demultiplexer::new(8);

    demux.set_chip_select(true);
    demux.set_selector(1);
    demux.set_input_word(10);

    println!("Mux output: {}", demux.read_word(1));
}
