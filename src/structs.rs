use std::fmt;

use rust_decimal::Decimal;

pub struct Payment {
    pub id: u32,
    pub amount: Decimal,
    pub payed_by: String,
}

#[derive(Debug)]
pub struct Liability {
    pub this_one: String,
    pub amount: Decimal,
    pub to: String,
    pub because_of_payment_id: u32,
}

impl fmt::Display for Liability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} owes {} to {} because of payment {}", self.this_one, self.amount, self.to, self.because_of_payment_id)
    }
}