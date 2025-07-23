#[derive(Debug)]
pub enum CashError {
    InsufficientFunds { available: u32, requested: u32 },
    InvalidAmount(i32),
}
