#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn copy_user(&self) -> User {
        User {
            username: self.username.clone(),
            email: self.email.clone(),
            sign_in_count: self.sign_in_count,
            active: self.active,
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {

    let user1 = User {
        username: String::from("user1"),
        email: String::from("ciao@ciao.it"),
        sign_in_count: 1,
        active: true,
    };
    
    // let user2 = User {
    //     username: String::from("user2"),
    //     email: String::from("ciao@ciao.it"),
    //     ..user1
    // }; // this will move the memory on the heap of user1 to user2, so user1 will be invalid (remember that String doesn't implement the Copy trait likewise int, ...)

    let mut user2 = user1.copy_user(); // copy_user(&user1) is the same as user1.clone(), but the function clone is not been implemented for User struct
    user2.username = String::from("user2"); // user2.username is a new String, so it's not a reference to user1.username

    println!("user1: {}, {}, {}, {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("user2: {}, {}, {}, {}", user2.username, user2.email, user2.sign_in_count, user2.active);
    // this is printing the debugging representation of the structure, it works because we have derived the trait in the struct
    println!("Printing user with debug: {user1:?}");

    dbg!(&user1);

    println!("Printing user with debug: {:#?}", user1);

    let rect1 = Rectangle { 
        width: 30, 
        height: 50 };
    let rect2 = Rectangle { 
        width: 10, 
        height: 40 };
    let rect3 = Rectangle { 
        width: 60, 
        height: 45 };
    let square = Rectangle::square(10);

    println!("The area of the rectangle is: {}", rect1.area());
    println!("The first rectangle can hold the second one: {}", rect1.can_hold(&rect2));
    println!("The first rectangle can hold the third one: {}", rect1.can_hold(&rect3));
    println!("The square is: {:?}", square);
}
