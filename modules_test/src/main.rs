pub mod pasquale {
    pub mod giulio {
        pub fn run () {
            println!("ciao")
        }
    }
}

mod antonio;

use antonio::zassiamo;

fn main() {
    pasquale::giulio::run();

    zassiamo::zasso();
}
