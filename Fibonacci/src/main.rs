use std::io;

fn main() {
    let mut len_series = String::new();

    println!("Enter the length of the series: ");
    
    io::stdin()
        .read_line(&mut len_series)
        .expect("Failed to read line");
    
    if len_series.trim() == "exit" {
        println!("Exiting...");
        return;
    }

    let len_series: u32 = len_series
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("You entered the length of the series as: {}", len_series);

    let mut series: u32 = 1;
    let mut last_num: u32 = 1;
    
    println!("Fibonacci's value at iteration ({}): {}",0, series);

    for i in 0..len_series {
        series = series + last_num;
        last_num = series - last_num;
        println!("Fibonacci's value at iteration ({}): {}",i+1, series);
    }
    }
