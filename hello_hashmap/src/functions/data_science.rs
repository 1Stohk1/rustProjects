use rand::{thread_rng, Rng};
use std::collections::HashMap;

// This main is using hashmap in order to get the frequencies of a vector

const LENGHT: usize = 11;
pub fn calculate() {
    let mut numbers: Vec<u8> = Vec::new();
    let mut frequencies: HashMap<u8, u32> = HashMap::new();

    for _ in 0..LENGHT {
        let mut rng = thread_rng();
        let x: u8 = rng.gen();
        numbers.push(x);
        let count = frequencies.entry(x).or_insert(0);
        *count += 1;
    }
    numbers.sort();
    println!("{numbers:?}");

    let median = match LENGHT % 2 {
        0 => (numbers[LENGHT / 2] + numbers[(LENGHT / 2) - 1]) / 2,
        _ => numbers[LENGHT / 2],
    };

    println!("The median of the distribution is: {}", median);

    println!("{frequencies:?}")
}
