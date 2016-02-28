#[derive(Debug, PartialEq)]
pub enum FizzRust {
    Value(u64),
    Fizz,
    Rust,
    FizzRust,
}

pub fn to_fizz_rust(value: u64) -> FizzRust {
    match (value % 5, value % 3) {
        (0, 0) => { FizzRust::FizzRust }
        (0, _) => { FizzRust::Rust }
        (_, 0) => { FizzRust::Fizz }
        _ => { FizzRust::Value(value) }
    }
}
