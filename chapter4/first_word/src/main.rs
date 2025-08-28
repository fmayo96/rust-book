fn main() {
    let s = String::from("Hello world");
    let first = first_word(&s);
    println!("{}", first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // We have to iterate over bytes because there are chars that take
    // more than one byte, and we should index the string with bytes.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
