#[derive(
    Debug,
    PartialEq
)]
pub enum Flag {
    Insensitive,
}

pub mod grep;
pub mod cli;
pub mod matches;
