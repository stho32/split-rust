use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::structs::*;

pub fn group_liabilities(liabilities: Vec<Liability>) -> Vec<Liability> {
    let sums = collect_sums(liabilities);
    return convert_sums_to_liabilities(sums);
}

fn create_key(this_one: &String, to: &String) -> String {
    let mut result: String = String::from("");
    result.push_str(&this_one);
    result.push(';');
    result.push_str(&to);
    return result;
}

struct TwoPeople {
    person1: String,
    person2: String,
}

fn split_key(key: &String) -> Option<TwoPeople> {
    let split = key.split(";");

    let collection: Vec<&str> = split.collect();

    if collection.len() != 2 {
        return None;
    }

    return Some(TwoPeople {
        person1: String::from(collection[0]),
        person2: String::from(collection[1]),
    });
}

fn convert_sums_to_liabilities(sums: HashMap<String, Decimal>) -> Vec<Liability> {
    let mut result: Vec<Liability> = vec![];

    for (key, val) in sums.iter() {
        let option_of_two_people = split_key(key);

        match option_of_two_people {
            Some(two_people) => {
                result.push(Liability {
                    this_one: two_people.person1,
                    amount: val.clone(),
                    to: two_people.person2,
                    because_of_payment_id: 0,
                });
            }
            None => {}
        }
    }

    return result;
}

fn collect_sums(liabilities: Vec<Liability>) -> HashMap<String, Decimal> {
    let mut map = HashMap::new();

    for liability in liabilities {
        let key = create_key(&liability.this_one, &liability.to);

        match map.get_mut(&key) {
            Some(x) => {
                *x = *x + liability.amount;
            }
            None => {
                map.insert(key, liability.amount);
            }
        }
    }

    return map;
}

#[cfg(test)]
mod tests {
    use crate::structs::Liability;

    use super::*;

    #[test]
    fn when_liabilites_concern_the_same_people_they_are_summed_up() {
        let liabilites: Vec<Liability> = vec![
            Liability {
                this_one: String::from("Person1"),
                amount: dec![10],
                to: String::from("Person2"),
                because_of_payment_id: 1,
            },
            Liability {
                this_one: String::from("Person1"),
                amount: dec![5],
                to: String::from("Person2"),
                because_of_payment_id: 2,
            },
        ];

        let result = group_liabilities(liabilites);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].amount, dec![15]);
    }

    #[test]
    fn when_liabilites_do_not_concern_the_same_people_they_are_not_summed_up() {
        let liabilites: Vec<Liability> = vec![
            Liability {
                this_one: String::from("Person1"),
                amount: dec![10],
                to: String::from("Person2"),
                because_of_payment_id: 1,
            },
            Liability {
                this_one: String::from("Person3"),
                amount: dec![5],
                to: String::from("Person2"),
                because_of_payment_id: 2,
            },
        ];

        let result = group_liabilities(liabilites);

        assert_eq!(result.len(), 2);
    }
}
