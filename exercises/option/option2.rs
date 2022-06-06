// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM NOT DONE

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    let word = if let Some(word) = optional_word {
        println!("The word is: {}", word);
        word
    } else {
        println!("The optional word doesn't contain anything");
        String::from(' ')
    };

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    let integer = if let integer = optional_integers_vec.pop() {
        println!("current value: {:?}", integer);
    };
}
