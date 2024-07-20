use std::boxed;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            value: val,
            next: None,
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
    tail: *mut Node,
    size: usize,
}

impl LinkedList {
    fn new(node: Node) -> Self {
        let mut head_ptr = Box::new(node);
        let tail_ptr = &mut *head_ptr as *mut Node;

        LinkedList {
            head: Some(head_ptr),
            tail: tail_ptr,
            size: 0 as usize,
        }
    }

    fn append(&mut self, val: i32) -> () {
        let new_tail = Box::new(Node::new(val));
        let new_tail_ptr = &*new_tail as *const Node as *mut Node;

        if self.head.is_none() {
            self.head = Some(new_tail);
            self.tail = new_tail_ptr;
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = new_tail_ptr;
        self.size += 1
    }

    fn append_front(&mut self, val: i32) -> () {
        let mut new_head = Box::new(Node::new(val));
        let new_tail_ptr = &*new_head as *const Node as *mut Node;

        if self.head.is_none() {
            self.head = Some(new_head);
            self.tail = new_tail_ptr;
        } else {
            new_head.next = self.head.take();
            self.head = Some(new_head);
        }

        self.size += 1;
    }
}

fn main() {
    println!("Hello, world!");
}
