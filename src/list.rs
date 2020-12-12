use std::collections::{HashMap, LinkedList};
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
    fn from_iter<I: IntoIterator<Item=T>>(_iter: I) -> Self {
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

pub fn length_of_longest_substring(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut indices = HashMap::new();
    let mut ret:usize = 0;
    let mut i:usize = 0;

    for j in 0..s.len() {
        match indices.insert(s[j], j) {
            Some(x) => i = i.max(x + 1),
            None => (),
        }
        ret = ret.max(j - i + 1);
    }

    ret as i32
}

pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    const M: i32 = 1_000_000_007;
    let mut dp = vec![vec![vec![0; 16]; 7]; n as usize];
    (1..=6).for_each(|j| dp[0][j][0] = 1);
    let mut ret = 0;

    for i in 1..(n as usize) {
        for j in 1..=6 {
            let f = |x: usize| {
                dp[i - 1][x][..(roll_max[x - 1] as usize)]
                    .iter()
                    .fold(0, |acc, x| (acc + x) % M)
            };
            dp[i][j][0] = (1..=6)
                .filter(|&x| x != j)
                .map(|x| f(x))
                .fold(0, |acc, x| (acc + x) % M);
            for k in 1..(roll_max[j - 1] as usize).min(i + 1) {
                dp[i][j][k] = dp[i - 1][j][k - 1];
            }
        }
    }

    for j in 1..=6 {
        for k in 0..(roll_max[j - 1].min(n) as usize) {
            ret = (ret + dp[n as usize - 1][j][k]) % M;
        }
    }

    ret
}
