fn main() {
    let mut s = String::from("hello");
    change(&mut s); // ref type must be aligned with data type,
                    // i.e. s and &s must be mutable together or immutable together
    println!("{:?}", s);

    let s2 = String::from("hello");
    cannot_change(&s2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn cannot_change(some_string: &String) {
    println!("ref: {:?}", some_string);
}
