pub mod common;
pub mod execution_unit;
pub mod machine;
pub mod memory;
pub mod processor;

use memory::file::File;
use crate::common::trigger_driver::TriggerMode;
use crate::memory::file::Command;
use crate::memory::register::PIPORegister;

fn test_register_pipo() {
    let mut pipo = PIPORegister::new(5, TriggerMode::RisingEdge);

    pipo.add_change_listener(|register| {
        println!("Change");
        for byte in register.byte_buffer.iter() {
            println!("{}", byte);
        }
    });

    println!("Should trigger change");
    pipo.set_byte(0, 0);
    pipo.set_clock_state(true);
    pipo.set_clock_state(false);

    println!("Should trigger change");
    pipo.set_byte(0, 0);
    pipo.set_clock_state(true);
    pipo.set_clock_state(false);

    println!("Should trigger change");
    pipo.set_byte(0, 0);
    pipo.set_clock_state(true);
    pipo.set_clock_state(false);
}

fn main() {
    // test_register_pipo();
    test_file();
}

fn test_file() {
    let mut general_purpose_registers = File::new(5, TriggerMode::RisingEdge);

    general_purpose_registers.add_output_listener(|file| {
        println!("Read");
        for byte in file.byte_buffer.byte_buffer.iter() {
            println!("{}", byte);
        }
    });

    general_purpose_registers.add_change_listener(|file| {
        println!("Change");
        for byte in file.byte_buffer.byte_buffer.iter() {
            println!("{}", byte);
        }
    });

    general_purpose_registers.set_job(Command::Write);

    println!("Should trigger change");
    general_purpose_registers.set_address(0);
    general_purpose_registers.set_byte(0);
    general_purpose_registers.set_clock_state(true);
    general_purpose_registers.set_clock_state(false);

    println!("Should trigger change");
    general_purpose_registers.set_address(1);
    general_purpose_registers.set_byte(1);
    general_purpose_registers.set_clock_state(true);
    general_purpose_registers.set_clock_state(false);

    println!("Should trigger change");
    general_purpose_registers.set_address(2);
    general_purpose_registers.set_byte(2);
    general_purpose_registers.set_clock_state(true);
    general_purpose_registers.set_clock_state(false);

    general_purpose_registers.set_job(Command::Read);

    println!("Should trigger output");
    general_purpose_registers.set_address(0);
    general_purpose_registers.set_clock_state(true);
    general_purpose_registers.set_clock_state(false);
}
