use crate::emulator::memory::{MemoryController, StaticRandomAccessMemory, StaticRandomAccessMemoryDriver};
pub mod emulator;

fn main() {
    let block_a: StaticRandomAccessMemory<usize, usize> = StaticRandomAccessMemory::new(16);
    let mut blocks = [block_a];

    let mut ram = StaticRandomAccessMemoryDriver::new(&mut blocks);
    let mut memory_controller = MemoryController::new(4, &mut ram);

    memory_controller.set_chip_select(true, 0);
    memory_controller.set_write_enable(true, 0);
    memory_controller.set_chip_select(true, 1);
    memory_controller.set_write_enable(true, 1);

    memory_controller.set_address(0, 0);
    memory_controller.set_address(0, 1);
    memory_controller.set_feed(1, 0);
    memory_controller.set_feed(2, 1);

    memory_controller.set_clock(true);
    memory_controller.set_clock(false);
    memory_controller.set_clock(true);
    memory_controller.set_clock(false);

    println!("Core: 0, data: {}", memory_controller.read_word(0));
    println!("Core: 1, data: {}", memory_controller.read_word(1));
}
