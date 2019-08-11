use std::collections::LinkedList;

fn main() {
    let mut lst = LinkedList::new();
    lst.push_back(1);
    lst.push_back(2);
    lst.push_back(3);

    println!("{:?}", lst)
}
