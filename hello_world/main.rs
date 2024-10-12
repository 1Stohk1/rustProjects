use std::io;

fn main () {
    let array: [i8; 5] = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("Enter the index of the array you want to access: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index must be a number");
    
    println!("The value at index {} is {}", index, array[index]);
}