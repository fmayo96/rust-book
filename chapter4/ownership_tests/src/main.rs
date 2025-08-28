fn main() {
    let s = gives_ownership();
    println!("{s}");

    let mut s2 = String::from("This comes from main");
    s2 = takes_and_gives_back(s2);
    println!("{}", s2);
    changes_string(&mut s2);
    println!("{}", s2);
}

fn gives_ownership() -> String {
    String::from("This comes from a function")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn changes_string(s: &mut String) {
    s.push_str(", afret comma comes from function");
}
