#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ListNode {
    val: RefCell<i32>,
    next: Option<Rc<ListNode>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Rc<ListNode>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        Self { head: None }
    }

    fn push(&mut self, _val: i32) {
        let new_node = ListNode {
            val: RefCell::new(_val),
            next: match &self.head {
                Some(head) => Some(Rc::clone(&head)),
                None => None
            },
        };
        self.head = Some(Rc::new(new_node));
    }
}

fn traverse() {
    let mut lst = LinkedList::new();
    println!("{:?}", &lst);
    lst.push(2);
    lst.push(3);
    println!("{:?}", &lst)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        traverse();
        assert_eq!(2 + 2, 4);
    }
}
