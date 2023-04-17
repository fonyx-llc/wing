use num_traits::{Num, Zero};
pub mod sr_latch;

pub trait WordNumber: Num + Zero {}