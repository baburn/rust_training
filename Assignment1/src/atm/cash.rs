pub struct Cash {
    pub total: Box<u32>, // heap allocated
}

impl Cash {
    pub fn new(amount: u32) -> Self {
        println!(
            "Cash created at heap address: {:p}",
            &*Box::new(amount)
        );
        Cash {
            total: Box::new(amount),
        }
    }
}
