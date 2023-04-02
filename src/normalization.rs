use rust_decimal_macros::dec;

use crate::structs::*;

pub fn normalize_liability(liability: Liability) -> Liability {
    if liability.this_one > liability.to {
        let opposite_amount = dec!(-1) * liability.amount;
        
        return Liability {
            this_one: liability.to,
            amount : opposite_amount,
            to : liability.this_one,
            because_of_payment_id : liability.because_of_payment_id
        };
    }

    return liability;
}

pub fn normalize_liabilities(liabilities: Vec<Liability>) -> Vec<Liability> {
    let mut result = Vec::new();

    for liability in liabilities {
        result.push(
            normalize_liability(liability)
        );
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::structs::Liability;

    use super::*;

    #[test]
    fn when_a_ows_b_the_liability_is_not_changed() {
        let liability = Liability {
            this_one: String::from("a"),
            to: String::from("b"),
            amount: dec!(10),
            because_of_payment_id: 1
        };

        let normalized = normalize_liability(liability);

        assert_eq!("a", normalized.this_one);
        assert_eq!("b", normalized.to);
        assert_eq!(dec!(10), normalized.amount);
    }

    #[test]
    fn when_b_ows_a_the_liability_is_turned_around() {
        let liability = Liability {
            this_one: String::from("b"),
            to: String::from("a"),
            amount: dec!(10),
            because_of_payment_id: 1
        };

        let normalized = normalize_liability(liability);

        assert_eq!("a", normalized.this_one);
        assert_eq!("b", normalized.to);
        assert_eq!(dec!(-10), normalized.amount);
    }

}