fn main() {
    let mut times = 2;
    {
        let mut borrow = |x| times += x;
        borrow(5)
    }
    assert_eq!(times, 7);
    let mut own = move |x| times += x;
    own(100);
    assert_eq!(times, 7);
}
