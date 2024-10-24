use std::thread;


fn main() {
    let list = vec![1, 2, 3];
    println!("This is a vector {:?}", list);

    thread::spawn(move || println!("This is the same vector from a thread {:?}", list))
        .join()
        .unwrap();
    
}
