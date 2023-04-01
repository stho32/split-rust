use rust_decimal::Decimal;
use crate::structs::*;

pub fn calculate_detailed_liabilities(group: Vec<String>, payments: Vec<Payment>) -> Vec<Liability> {
    let mut liabilities: Vec<Liability> = vec![];

    for payment in payments {
        let amount_per_member = (payment.amount / Decimal::from(group.len())).round_dp(2);

        for member in &group {
            if member == &payment.payed_by {
                continue;
            }

            let liability = Liability {
                because_of_payment_id: payment.id,
                amount: amount_per_member,
                this_one: member.clone(),
                to: payment.payed_by.clone(),
            };

            liabilities.push(liability);
        }
    }

    return liabilities;
}