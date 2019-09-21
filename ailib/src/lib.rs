#[macro_use]
extern crate derive_builder;

mod winner;
mod state;
mod deterministic_state;
mod strategy;

pub mod strategies;

pub use winner::*;
pub use state::*;
pub use deterministic_state::*;
pub use strategy::*;





pub trait Rated {
    type Rating: Ord + Copy;
    fn rating(&self) -> Self::Rating;
}
