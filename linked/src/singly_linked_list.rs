#![allow(unused)]

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Debug)]
pub struct ListNode<T> {
    val: T,
    next: Link<T>,
}

impl<T> ListNode<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode {
            val: elem,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, _val: T) {
        let new_node = ListNode::new(_val);
        match self.tail.take() {
            Some(last) => {
                last.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node)
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node)
            }
        }
    }

    fn peek(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    fn peek_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
    }
}

fn traverse() {
    let mut lst = LinkedList::new();
    println!("lst = {:?}", &lst);
    lst.push(2);
    lst.push(3);
    println!("lst = {:?}", &lst.head);
    println!("lst = {:?}", &lst.tail);
    println!("last = {:?}", &lst.peek());
    *lst.peek_mut().unwrap() = 4;
    println!("last = {:?}", &lst.tail)
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
