use rust_decimal_macros::dec;

use crate::structs::*;

pub fn denormalize_liability(liability: Liability) -> Liability {
    if liability.amount < dec!(0) {
        let opposite_amount = dec!(-1) * liability.amount;

        return Liability {
            this_one: liability.to,
            amount: opposite_amount,
            to: liability.this_one,
            because_of_payment_id: liability.because_of_payment_id,
        };
    }

    return liability;
}

pub fn denormalize_liabilities(liabilities: Vec<Liability>) -> Vec<Liability> {
    let mut result = Vec::new();

    for liability in liabilities {
        result.push(denormalize_liability(liability));
    }

    return result;
}
