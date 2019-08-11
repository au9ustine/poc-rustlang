use std::collections::LinkedList;

fn main() {
    // create a linked list
    let mut lst = LinkedList::new();
    lst.push_back(1);
    lst.push_back(2);
    lst.push_back(3);
    println!("{:?}", lst);

    // traverse a step
    lst.iter().next();
    println!("{:?}", lst.front());

    // traversal with modification
    for (i, v) in lst.iter_mut().enumerate() {
        match i {
            0 => *v = 4,
            _ => ()
        }
    };
    println!("{:?}", lst.front())

}
