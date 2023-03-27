use rust_decimal::Decimal;
use rust_decimal_macros::dec;

struct Payment {
    id: u32,
    amount: Decimal,
    payed_by: String,
}

#[derive(Debug)]
struct Liability {
    because_of_payment_id: u32,
    amount: Decimal,
    this_one: String,
    to: String,
}

fn main() {
    let group = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];

    let payments = vec![Payment {
        id: 1,
        amount: dec!(10.00),
        payed_by: "Alice".to_string(),
    }];

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

    println!("{:?}", liabilities);
}
