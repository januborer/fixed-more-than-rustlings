// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// I AM NOT DONE

fn main() {
    let mut answer = String::from("hello");
    let ref1 = &mut answer;
    println!("My current favorite color is {}", ref1);
    let ref2 = &mut answer;
    ref2.push_str(" Borer");
    println!("My current favorite color is {}", ref2);
}

fn current_favorite_color() -> String {
    String::from("blue")
}
