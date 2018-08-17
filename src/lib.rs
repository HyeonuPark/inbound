
extern crate itertools;

#[cfg(test)]
extern crate rand;

mod slice;
mod index;
pub use self::slice::Slice;
pub use self::index::Index;

pub mod algorithm;

#[cfg(debug_assertions)]
mod puid {
    extern crate snowflake;

    pub use self::snowflake::ProcessUniqueId as Id;
}
