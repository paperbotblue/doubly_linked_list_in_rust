use std::sync::Mutex;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
struct Node<T> {
    next: Option<Arc<Mutex<Node<T>>>>,
    prev: Option<Arc<Mutex<Node<T>>>>,
    data: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            next: None,
            prev: None,
            data: value,
        }
    }
}

struct Deque<T> {
    begin: Option<Arc<Mutex<Node<T>>>>,
    end: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> Deque<T>
where
    T: Debug + PartialEq,
{
    fn new() -> Deque<T> {
        Deque {
            begin: None,
            end: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Arc::new(Mutex::new(Node::new(value)));
        match &self.begin {
            Some(node) => {
                new_node.lock().unwrap().next = Some(node.clone());
                self.begin = Some(new_node);
            }
            None => {
                assert!(self.end.is_none());
                self.begin = Some(new_node.clone());
                self.end = Some(new_node);
            }
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Arc::new(Mutex::new(Node::new(value)));
        match &self.end {
            Some(node) => {
                new_node.lock().unwrap().prev = Some(node.clone());
                node.lock().unwrap().next = Some(new_node.clone());
                self.end = Some(new_node);
            }
            None => {
                assert!(self.end.is_none());
                self.begin = Some(new_node.clone());
                self.end = Some(new_node);
            }
        }
    }

    fn drop(&mut self, value: T) {
        let mut ptr = self.begin.clone();
        let mut ptr_prev: Option<Arc<Mutex<Node<T>>>> = None;
        while let Some(node) = ptr.clone() {
            if node.lock().unwrap().data == value {
                if ptr_prev.is_none() {
                    self.begin = self.begin.clone().unwrap().lock().unwrap().next.clone();
                } else {
                    ptr_prev.clone().unwrap().lock().unwrap().next =
                        node.lock().unwrap().next.clone();
                    node.lock().unwrap().prev = ptr_prev;
                }
                break;
            }
            ptr_prev = ptr.clone();
            ptr = ptr.unwrap().lock().unwrap().next.clone();
        }
    }
}

fn main() {
    let mut xs = Deque::<i32>::new();
    xs.push_back(2);
    xs.push_back(3);
    xs.push_back(4);
    xs.push_back(5);
    xs.drop(2);
    println!("{:?}", xs);
}

impl<T> Debug for Deque<T>
where
    T: Debug,
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut p = self.begin.clone();

        while p.is_some() {
            match p {
                Some(ref node) => {
                    println!("{:?}", node.lock().unwrap().data);
                    p = p.unwrap().lock().unwrap().next.clone();
                }
                None => {}
            }
        }

        Ok(())
    }
}
