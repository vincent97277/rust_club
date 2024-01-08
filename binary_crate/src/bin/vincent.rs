
use vincent::generatorIdCard::{IdCard};

fn main() {
    let mut id_card = IdCard::new();
    id_card.generate();

    println!("id_card is: {}", id_card.id_number);
}
