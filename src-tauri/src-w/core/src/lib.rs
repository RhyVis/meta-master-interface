#[allow(dead_code)]
pub mod data;
#[allow(dead_code)]
pub mod util;

pub enum Or<A, B> {
    This(A),
    That(B),
}
