pub mod data;
pub mod util;

pub enum Or<A, B> {
    This(A),
    That(B),
}
