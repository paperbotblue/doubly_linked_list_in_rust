use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Node<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
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
    begin: Option<Rc<RefCell<Node<T>>>>,
    end: Option<Rc<RefCell<Node<T>>>>,
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
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match &self.begin {
            Some(node) => {
                new_node.borrow_mut().next = Some(node.clone());
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
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match &self.end {
            Some(node) => {
                new_node.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(new_node.clone());
                self.end = Some(new_node);
            }
            None => {
                assert!(self.end.is_none());
                self.begin = Some(new_node.clone());
                self.end = Some(new_node.into());
            }
        }
    }

    fn drop(&mut self, value: T) {
        let mut ptr = self.begin.clone();
        let mut ptr_prev: Option<Rc<RefCell<Node<T>>>> = None;

        while let Some(node) = &ptr.clone() {
            if node.borrow().data == value {
                match ptr_prev {
                    Some(node_prev) => {
                        node_prev.borrow_mut().next = node.borrow().next.clone();
                        match &node.borrow().next {
                            Some(node_next) => node_next.borrow_mut().prev = Some(node_prev),
                            None => {}
                        }
                    }
                    None => self.begin = self.begin.clone().unwrap().borrow().next.clone(),
                }
                break;
            }

            ptr_prev = Some(node.clone());
            ptr = node.borrow().next.clone();
        }
    }
}

fn main() {
    let mut xs = Deque::<i32>::new();
    xs.push_front(1);
    xs.push_front(2);
    xs.push_back(2);
    xs.push_back(2);
    xs.push_front(1);
    xs.drop(1);
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
                    println!("{:?}", node.borrow().data);
                    p = p.unwrap().borrow().next.clone();
                }
                None => {}
            }
        }

        Ok(())
    }
}
