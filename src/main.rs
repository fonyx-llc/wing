pub mod memory;

fn main() {
    let mut register = memory::register::PIPORegister::new(2, memory::register::TriggerMode::RisingEdge);

    register.add_change_listener(|register| {
        print!("Val: ");
        for byte in register.byte_buffer.iter() {
            print!(" ");
            for bit in 0..8 {
                print!("{}", (byte >> bit) & 1);
            }
        }
        println!();
    });

    register.set_byte(0xF1, 0);
    register.set_clock_state(true);
    register.set_clock_state(false);
}
