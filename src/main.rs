fn main() {
    let hello = "hello world";
    println!("Before string: {}", hello);
    let word = get_first_word(hello);
    println!("After string: {}", word);
}

fn get_first_word(s: &str) -> &str {
    let s_bytes = s.as_bytes();

    for (index, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index]
        }
    }
    &s[..]
}
