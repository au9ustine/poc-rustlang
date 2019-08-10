#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, _val: i32) {
        let new_node = ListNode {
            val: _val,
            next: None,
        };
        match &self.tail {
            Some(last) => last.borrow_mut().next = Some(Rc::new(RefCell::new(new_node))),
            None => self.tail = Some(Rc::new(RefCell::new(new_node))),
        }
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
