use crate::emulator::memory::SingleStaticRandomAccessMemory;
pub mod emulator;

fn main() {
    let mut ssram: SingleStaticRandomAccessMemory<u8, u8> = SingleStaticRandomAccessMemory::new(8);

    ssram.set_chip_select(false);
    ssram.set_write_enable(true);

    ssram.set_address(0);
    ssram.set_feed(0x42);
    
    ssram.set_clock(true);
    ssram.set_clock(false);
    
    ssram.set_chip_select(true);
    ssram.set_write_enable(false);

    println!("Data at address 0: {}", ssram.read_word());
}
