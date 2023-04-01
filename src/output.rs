use crate::structs::Liability;

pub fn print_liabilities(liabilities : &Vec<Liability>) {
    for liability in liabilities {
        println!("{}", liability);
    }
}
