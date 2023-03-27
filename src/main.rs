use rust_decimal::Decimal;
use rust_decimal_macros::dec;

struct Payment {
    id: u32,
    amount: Decimal,
    payed_by: String,
}

struct Liability {
    because_of_payment_id: u32,
    amount: Decimal,
    to: String,
}

struct Member {
    name: String,
    liabilities: Vec<Liability>,
}

struct Group {
    members: Vec<Member>,
}

fn main() {
    let group = Group {
        members: vec![
            Member {
                name: "Alice".to_string(),
                liabilities: vec![],
            },
            Member {
                name: "Bob".to_string(),
                liabilities: vec![],
            },
            Member {
                name: "Charlie".to_string(),
                liabilities: vec![],
            },
        ],
    };

    let payments = vec![Payment {
        id: 1,
        amount: dec!(10.00),
        payed_by: "Alice".to_string(),
    }];

    for payment in payments {
        let payed_by = &mut group
            .members
            .iter()
            .find(|member| member.name == payment.payed_by)
            .unwrap();

        let amount_per_member = payment.amount / Decimal::from(group.members.len());

        for member in &group.members {
            if member.name == payed_by.name {
                continue;
            }

            let liability = Liability {
                because_of_payment_id: payment.id,
                amount: amount_per_member,
                to: member.name.clone(),
            };

            payed_by.liabilities.push(liability);
        }
    }

    for member in group.members {
        println!("{}:", member.name);
        for liability in member.liabilities {
            println!(
                "  {} has to pay {} because of payment {}",
                liability.to, liability.amount, liability.because_of_payment_id
            );
        }
    }
}
