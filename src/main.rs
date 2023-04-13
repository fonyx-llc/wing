pub mod common;
pub mod execution_unit;
pub mod machine;
pub mod memory;
pub mod processor;
pub mod wire;

fn main() {
    wire::main::handle();
}