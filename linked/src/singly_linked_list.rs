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
}

impl LinkedList {
    fn new() -> LinkedList {
        Self { head: None }
    }

    fn push(&mut self, _val: i32) {
        let new_node = Rc::new(RefCell::new(ListNode {
            val: _val,
            next: None,
        }));
        match &self.head {
            Some(last) => {
                last.borrow_mut().next = Some(Rc::clone(&new_node));
            }
            None => self.head = Some(Rc::clone(&new_node)),
        }
    }

    fn last(&mut self) {
        let mut current = self.head.as_ref().map(|head| Rc::clone(&head));
    }
}

fn traverse() {
    let mut lst = LinkedList::new();
    println!("lst = {:?}", &lst);
    lst.push(2);
    lst.push(3);
    println!("lst = {:#?}", &lst.head);
    lst.last()
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
