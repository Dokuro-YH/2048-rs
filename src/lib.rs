/// A mask with a single section of 16 bits set to 0.
/// Used to extract a "horizontal slice" out of a 64 bit integer.
pub static ROW_MASK: u64 = 0xFFFF;

/// A `u64` mask with 4 sections each starting after the n * 16th bit.
/// Used to extract a "vertical slice" out of a 64 bit integer.
pub static COL_MASK: u64 = 0x000F_000F_000F_000F_u64;

mod game;
mod moves;

pub use game::Game;
pub use moves::Direction;