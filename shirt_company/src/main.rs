#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway (&self, userpreference: Option<ShirtColor>) -> ShirtColor {
        userpreference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked (&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref);

    println!("The user with preference {:?} gets the shirt {:?}",
    user_pref, giveaway);

    
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with preference {:?} gets the shirt {:?}",
    user_pref2, giveaway2);
}
