pub mod data;
pub mod util;

pub enum Whether<A, B> {
    This(A),
    That(B),
}

pub const DIR_ARCHIVE: &str = "archive";
