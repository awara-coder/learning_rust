fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    let s1 = String::from("string 1");
    take_owernship(s1);
    // Error as the ownership of s is transfered to take_ownership()
    // println!("{s1}");

    slice_experiments();
}

fn take_owernship(s: String) {
    println!("{s}")
}

// fn modify_s(s: &String) {
//     s.push_str("some string");
// }

fn slice_experiments() {
    let s = String::from("hello world, it is nice weather today!");
    // let hello = &s[0..5];
    // let world = &[6..11];
    let first_word_of_s = first_word(&s);
    println!("First word of s is {first_word_of_s}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &value) in bytes.iter().enumerate() {
        // Check if value is a space
        if value == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
