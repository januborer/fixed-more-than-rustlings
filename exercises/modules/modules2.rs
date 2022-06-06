// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

// I AM NOT DONE

mod delicious_snacks {

    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
    mod test {
        const TEST: &'static str = "Test";
        pub fn test() {
            println!("{}", TEST);
        }
    }

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
        pub fn print_test() {
            super::test::test();
        }
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

use delicious_snacks::fruit;
use delicious_snacks::veggie;

fn main() {
    delicious_snacks::fruits::print_test();
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
