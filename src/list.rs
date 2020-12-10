use std::collections::LinkedList;
use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node::new(_element, self.head.take())));
        self.len = self.len + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let mut n = self.head.take().unwrap();
        self.head = n.next.take();
        self.len = self.len - 1;
        Some(n.data)
    }

    pub fn peek(&self) -> Option<&T> {
        // Some(&self.head.as_ref().unwrap().data)
        match self.head {
            Some(ref _node) => Some(&_node.data),
            None => None,
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut cur_link = self;
        while let Some(x) = cur_link.pop() {
            list.push(x);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();
        for x in _iter {
            list.push(x);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::new();
        let mut cur_link = self;
        while let Some(x) = cur_link.pop() {
            v.insert(0, x);
        }
        v
    }
}
