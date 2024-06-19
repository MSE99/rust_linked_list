use std::{fmt::Debug, rc::Rc};

#[derive(Debug)]
struct Node<T>
where
    T: Sized + Copy,
{
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T>
where
    T: Sized + Copy,
{
    fn new(v: T) -> Self {
        Node {
            value: v,
            next: None,
        }
    }
}

fn append<T>(s: &Node<T>, v: T) -> Node<T>
where
    T: Sized + Copy + Debug,
{
    match &s.next {
        Some(node) => Node {
            value: node.value,
            next: Some(Rc::new(append(&node, v))),
        },
        None => Node {
            value: s.value,
            next: Some(Rc::new(Node::new(v))),
        },
    }
}

fn main() {
    let l: Node<i32> = Node::new(30);
    let appended: Node<i32> = append(&l, 40);
    let appended_twice: Node<i32> = append(&appended, 50);

    println!("{:?}", appended_twice);
    println!("{:?}", appended);
    println!("{:?}", l);
}
