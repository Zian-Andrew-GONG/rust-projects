fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}\nworld: {}", hello, world);

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("slice: {}", slice);
    let slice = &s[..2];
    println!("slice: {}", slice);

    let len = s.len();
    let slice = &s[0..len];
    println!("slice: {}", slice);
    let slice = &s[..];
    println!("slice: {}", slice);

    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error!
    println!("the first word is: {}", word);

    let mut s = String::from("hello world");
    s.clear();

    let s = "Hello, world!";
    let word = first_word_1(&s);
    println!("the first word is: {}", word);
    let s = String::from("hello world");
    let word = first_word_1(&s);
    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn first_word_1(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
