fn main() {
    let uindexes: Vec<usize> = "hello".chars().enumerate().map(|x| x.0).collect();
    let indexes: Vec<i32> = (0..).zip("hello".chars()).map(|x| x.0).collect();
    println!("{:?}", uindexes);
    println!("{:?}", indexes);
}
