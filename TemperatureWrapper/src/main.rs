use std::io;

fn converter(temperature: &mut u32) {
    *temperature = (*temperature - 32) * 5 / 9;
}

fn main() {
    let mut farhenheit = String::new();

    println!("Enter the temperature in Farhenheit: ");

    io::stdin()
        .read_line(&mut farhenheit)
        .expect("Failed to read line");

    let farhenheit: u32 = farhenheit
        .trim()
        .parse()
        .expect("Please type a number!");

    let mut farhenheit = Box::new(farhenheit);
    
    converter(&mut farhenheit);

    println!("The temperature in Celsius is: {}", farhenheit);
}
