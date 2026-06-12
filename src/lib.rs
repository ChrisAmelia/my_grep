#[derive(
    Debug,
    PartialEq
)]
pub enum Flag {
    Insensitive,
    ShowLineNumber,
}

pub mod grep;
pub mod cli;
