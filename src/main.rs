use normalization::normalize_liabilities;
use rust_decimal_macros::dec;

mod structs;
mod payments_to_liabilities;
mod output;
mod normalization;

use structs::*;
use payments_to_liabilities::calculate_detailed_liabilities;


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

    let liabilities = calculate_detailed_liabilities(group, payments);
    let liabilities = normalize_liabilities(liabilities);
    
    output::print_liabilities(&liabilities);
}
