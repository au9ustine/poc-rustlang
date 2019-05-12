use std::env;

fn main() {
    let keys: Vec<String> = env::vars_os()
        .map(|p| p.0.into_string().ok().unwrap())
        .collect();
    println!("{:#?}", keys);
}
