//! Memory debug utilities which can be used for printing memory buffers in the serial host,
//! terminal, or other debug output.

use super::ff::FF;

/// # Arguments
///  - ffs: (Flip Flops) A vector of flip flops to print.
pub fn print_ffs(ffs: Vec<FF>) {
	for ff in ffs {
		print!("{}", if ff.bit { "▓" } else { "a" });
	}
	println!();
}