use crate::emulator::memory::{StaticRandomAccessMemory, StaticRandomAccessMemoryDriver};
pub mod emulator;

fn main() {
    let block_a: StaticRandomAccessMemory<usize, usize> = StaticRandomAccessMemory::new(1024);
    let block_b: StaticRandomAccessMemory<usize, usize> = StaticRandomAccessMemory::new(1024);
    let block_c: StaticRandomAccessMemory<usize, usize> = StaticRandomAccessMemory::new(1024);
    let block_d: StaticRandomAccessMemory<usize, usize> = StaticRandomAccessMemory::new(1024);
    let mut blocks = [block_a, block_b, block_c, block_d];

    let mut ram = StaticRandomAccessMemoryDriver::new(&mut blocks);

    ram.set_chip_select(true);
    ram.set_write_enable(true);

    ram.set_address(1000);
    ram.set_feed(127);

    ram.set_clock(true);
    ram.set_clock(false);

    let data = ram.read_word();
    println!("Data: {}", data);

    ram.set_address(1);
    ram.set_feed(255);

    ram.set_clock(true);
    ram.set_clock(false);

    let data = ram.read_word();
    println!("Data: {}", data);
}
