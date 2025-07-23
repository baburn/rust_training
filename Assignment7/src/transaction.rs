#[derive(Debug)]
pub enum Transaction {
    Deposit { account_id: u32, amount: u32 },
    Withdraw { account_id: u32, amount: u32 },
    Transfer { from: u32, to: u32, amount: u32 },
}
