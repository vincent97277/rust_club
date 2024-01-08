use rand::prelude::*;
use std::collections::HashMap;
use rand::Rng;

pub struct IdCard {
    pub id_number: String,
}

impl IdCard {
    pub fn new() -> IdCard {
        IdCard {
            id_number: String::new(),
        }
    }

    pub fn generate(&mut self) {
        let mut map = HashMap::new();

        let numbers = vec![10, 11, 12, 13, 14, 15, 16, 17, 34, 18, 19, 20, 21, 22, 35, 23, 24, 25, 26, 27, 28, 29, 32, 30, 31, 33];
        let letters: Vec<char> = ('A'..='Z').collect();

        for (letter, number) in letters.iter().zip(numbers.iter()) {
            map.insert(*number, *letter);
        }

        let mut rng = thread_rng();
        let area = numbers.choose(&mut rng).unwrap();
        let gender: i8 = rng.gen_range(1..=2);
        let serial_no:Vec<i32> = (0..7).map(|_| rng.gen_range(0..10)).collect();

        let temp_id_number = format!("{}{}{}", area, gender, serial_no.into_iter().map(|number| number.to_string()).collect::<String>());

        let check_digit = self.calculate_check_digit(&temp_id_number);

        self.id_number = format!(
            "{:}{}{}",
            map.get(&area).unwrap(), temp_id_number[2..].to_string(), check_digit
        );
    }

    fn calculate_check_digit(&self, input: &str) -> i32 {
        const MODULO: i32 = 10;
        let weights = vec![1, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let mut answer = 0;
        for (char, weight) in input.chars().map(|c| c.to_digit(10).unwrap()).zip(weights.iter()) {
            answer += char as i32 * weight;
        }

        match answer % MODULO {
            0 => 0,
            n => 10 - n,
        }
    }
}
