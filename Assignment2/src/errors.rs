use std::fmt;

#[derive(Debug)]
pub enum LoanError {
    Age(AgeError),
    Income(IncomeError),
}

#[derive(Debug)]
pub enum AgeError {
    InvalidAge(u32),
}

#[derive(Debug)]
pub enum IncomeError {
    InsufficientIncome { income: u32, required: u32 },
}

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoanError::Age(e) => write!(f, "Age Error: {}", e),
            LoanError::Income(e) => write!(f, "Income Error: {}", e),
        }
    }
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgeError::InvalidAge(age) => write!(f, "Applicant age {} is not within 21-60", age),
        }
    }
}

impl fmt::Display for IncomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IncomeError::InsufficientIncome { income, required } => {
                write!(f, "Income {} is less than required {}", income, required)
            }
        }
    }
}
