use super::Rdx;

use std::slice;
use std::fmt;


enum Node<T> where T: Rdx {
    Inner(NodeInner<T>),
    Child(T),
    Free,
}


struct NodeInner<T> where T: Rdx {
    round: usize,
    children: Vec<Node<T>>,
}


impl<T> NodeInner<T> where T: Rdx {
    fn new(round: usize, nbuckets: usize) -> NodeInner<T> {
        let mut children = Vec::with_capacity(nbuckets);
        for _ in 0..nbuckets {
            children.push(Node::Free);
        }
        NodeInner {
            round: round,
            children: children,
        }
    }

    fn insert(&mut self, x: T) {
        let bucket = x.get_bucket(self.round - 1);

        if self.round > 1 {
            let clen = self.children.len();
            let replace = match self.children[bucket] {
                Node::Free => {
                    let mut inner = NodeInner::new(self.round - 1, clen);
                    inner.insert(x);
                    Some(inner)
                },
                Node::Inner(ref mut inner) => {
                    inner.insert(x);
                    None
                },
                Node::Child(_) => unreachable!(),
            };

            if let Some(inner) = replace {
                self.children[bucket] = Node::Inner(inner);
            }
        } else {
            let alloc = match self.children[bucket] {
                Node::Free => true,
                Node::Child(_) => false,
                Node::Inner(_) => unreachable!(),
            };

            if alloc {
                self.children[bucket] = Node::Child(x);
            } else {
                match self.children[bucket] {
                    Node::Child(ref mut y) => *y = x,  // XXX: is that a good idea?
                    _ => unreachable!(),
                }
            }
        }
    }

    fn nnodes(&self) -> usize {
        let mut result = 1;
        for c in self.children.iter() {
            match c {
                &Node::Inner(ref inner) => {
                    result += inner.nnodes();
                },
                _ => {}
            }
        }
        result
    }
}


pub struct RdxTree<T> where T: Rdx {
    root: Node<T>,
}


impl<T> RdxTree<T> where T: Rdx {
    pub fn new() -> RdxTree<T> {
        let rounds = <T as Rdx>::cfg_nrounds();
        let buckets = <T as Rdx>::cfg_nbuckets();
        RdxTree {
            root: Node::Inner(NodeInner::<T>::new(rounds, buckets)),
        }
    }

    pub fn insert(&mut self, x: T) {
        match self.root {
            Node::Inner(ref mut inner) => {
                inner.insert(x);
            },
            _ => {
                unreachable!();
            }
        }
    }

    pub fn iter<'a>(&'a self) -> RdxTreeIter<'a, T> {
        let mut iters = Vec::new();
        match self.root {
            Node::Inner(ref inner) => {
                iters.push(inner.children.iter());
            },
            _ => unreachable!(),
        }
        RdxTreeIter {
            iters: iters,
        }
    }

    pub fn nnodes(&self) -> usize {
        match self.root {
            Node::Inner(ref inner) => {
                inner.nnodes()
            },
            _ => {
                unreachable!()
            }
        }
    }
}


pub struct RdxTreeIter<'a, T> where T: Rdx + 'a {
    iters: Vec<slice::Iter<'a, Node<T>>>,
}

impl<'a, T> Iterator for RdxTreeIter<'a, T> where T: Rdx + 'a {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let mut result: Option<&'a T> = None;

        while self.iters.len() > 0 && result.is_none() {
            let mut push: Option<slice::Iter<'a, Node<T>>> = None;
            let mut pop = false;

            if let Some(mut it) = self.iters.last_mut() {
                match it.next() {
                    Some(&Node::Free) => {},
                    Some(&Node::Child(ref x)) => {
                        result = Some(x);
                    },
                    Some(&Node::Inner(ref inner)) => {
                        push = Some(inner.children.iter());
                    },
                    None => {
                        pop = true;
                    },
                }
            } else {
                unreachable!();
            }

            if pop {
                self.iters.pop();
            } else if let Some(next) = push {
                self.iters.push(next);
            }
        }

        result
    }
}


fn print_node<T>(node: &Node<T>, depth: usize) where T: fmt::Display + Rdx {
    let prefix: String = (0..depth).map(|_| ' ').collect();
    match *node {
        Node::Inner(ref inner) => {
            for (i, c) in inner.children.iter().enumerate() {
                println!("{}{}:", prefix, i);
                print_node(c, depth + 1);
            }
        },
        Node::Child(ref x) => {
            println!("{}=> {}", prefix, x);
        },
        Node::Free => {
            println!("{}X", prefix);
        },
    }
}


fn print_tree<T>(tree: &RdxTree<T>) where T: fmt::Display + Rdx {
    print_node(&tree.root, 0);
}


#[test]
fn test_insert() {
    let mut tree: RdxTree<u32> = RdxTree::new();
    tree.insert(1);
    tree.insert(22);
    tree.insert(2);
    tree.insert(1024);
    tree.insert(0);

    let should = vec![0, 1, 2, 22, 1024];
    let is: Vec<u32> = tree.iter().cloned().collect();
    assert_eq!(should, is);
    assert_eq!(tree.nnodes(), 11);
}
