use super::Rdx;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::fmt;
use std::rc::Rc;


#[derive(Clone)]
enum Node<T> where T: Clone + Rdx {
    Inner(Rc<RefCell<NodeInner<T>>>),
    Pruned(Rc<RefCell<NodePruned<T>>>),
    Child(T),
    Free,
}


#[derive(Clone)]
enum NodeLimited<T> where T: Clone + Rdx {
    Inner(Rc<RefCell<NodeInner<T>>>),
    Child(T),
}


impl<'a, T> From<&'a NodeLimited<T>> for Node<T> where T: Clone + Rdx {
    fn from(obj: &'a NodeLimited<T>) -> Node<T> {
        match obj {
            &NodeLimited::Inner(ref inner) => Node::Inner(inner.clone()),
            &NodeLimited::Child(ref x) => Node::Child(x.clone()),
        }
    }
}


#[derive(Clone)]
struct NodeInner<T> where T: Clone + Rdx {
    round: usize,
    children: Vec<Node<T>>,
}


#[derive(Clone)]
struct NodePruned<T> where T: Clone + Rdx {
    round: usize,
    nbuckets: usize,
    buckets: Vec<usize>,
    child: NodeLimited<T>,
}


impl<T> NodeInner<T> where T: Clone + Rdx {
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
                    let pruned = Rc::new(RefCell::new(NodePruned::new(self.round - 1, clen, x)));
                    Some(Node::Pruned(pruned))
                },
                Node::Inner(ref mut inner) => {
                    inner.borrow_mut().insert(x);
                    None
                },
                Node::Pruned(ref mut pruned) => {
                    Some(pruned.borrow().insert_or_split(x))
                }
                Node::Child(_) => unreachable!(),
            };

            if let Some(obj) = replace {
                self.children[bucket] = obj;
            }
        } else {
            let alloc = match self.children[bucket] {
                Node::Free => true,
                Node::Child(_) => false,
                Node::Inner(_) => unreachable!(),
                Node::Pruned(_) => unreachable!(),
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

    fn nnodes(&self) -> (usize, usize, usize, usize) {
        let mut result = (1, 0, 0, 0);
        for c in self.children.iter() {
            match c {
                &Node::Inner(ref inner) => {
                    let tmp = inner.borrow().nnodes();
                    result.0 += tmp.0;
                    result.1 += tmp.1;
                    result.2 += tmp.2;
                    result.3 += tmp.3;
                },
                &Node::Pruned(ref pruned) => {
                    let tmp = pruned.borrow().nnodes();
                    result.0 += tmp.0;
                    result.1 += tmp.1;
                    result.2 += tmp.2;
                    result.3 += tmp.3;
                },
                &Node::Child(_) => {
                    result.2 += 1;
                },
                &Node::Free => {
                    result.3 += 1;
                },
            }
        }
        result
    }
}


impl<T> NodePruned<T> where T: Clone + Rdx {
    fn new(round: usize, nbuckets: usize, x: T) -> NodePruned<T> {
        let mut buckets = Vec::with_capacity(round);
        for i in 0..round {
            let r = round - i;
            let bucket = x.get_bucket(r - 1);
            buckets.push(bucket);
        }
        let child = NodeLimited::Child(x);
        NodePruned {
            round: round,
            nbuckets: nbuckets,
            buckets: buckets,
            child: child,
        }
    }

    fn insert_or_split(&self, x: T) -> Node<T> {
        for i in 0..self.buckets.len() {
            let r = self.round - i;
            let bucket_y = self.buckets[i];
            let bucket_x = x.get_bucket(r - 1);

            if bucket_x != bucket_y {
                // === outcome a: split ===
                //
                //     [head][middle/diff][tail]
                //
                // becomes
                //
                //                     |-[tail1]
                //     [head]-[middle]-|
                //                     |-[tail2]
                //

                // split head, middle and tail
                let mut buckets_head = self.buckets.clone();
                let buckets_tail = buckets_head.split_off(i + 1);
                buckets_head.pop();  // remove middle part

                // inner node = middle part
                let mut inner = NodeInner::new(self.round - buckets_head.len(), self.nbuckets);

                // add old tail and new branch to inner node
                if buckets_tail.is_empty() {
                    inner.children[bucket_y] = (&self.child).into();
                } else {
                    let tail = Rc::new(RefCell::new(NodePruned {
                        round: self.round - i - 1,
                        nbuckets: self.nbuckets,
                        buckets: buckets_tail,
                        child: self.child.clone(),
                    }));
                    inner.children[bucket_y] = Node::Pruned(tail);
                }
                inner.insert(x);

                // either return inner node (when head is empty) or create new head
                if buckets_head.is_empty() {
                    return Node::Inner(Rc::new(RefCell::new(inner)));
                } else {
                    let head = Rc::new(RefCell::new(NodePruned {
                        round: self.round,
                        nbuckets: self.nbuckets,
                        buckets: buckets_head,
                        child: NodeLimited::Inner(Rc::new(RefCell::new(inner))),
                    }));
                    return Node::Pruned(head);
                }
            }
        }

        // === outcome b: insert ===
        // INFO: Copying seems to be faster than returning an Option and do the change in-place.
        //       I don't know why this is the case.
        let mut cpy = self.clone();
        match cpy.child {
            NodeLimited::Inner(ref mut inner) => {
                inner.borrow_mut().insert(x);
            },
            NodeLimited::Child(ref mut y) => {
                *y = x;
            },
        }
        Node::Pruned(Rc::new(RefCell::new(cpy)))
    }

    fn nnodes(&self) -> (usize, usize, usize, usize) {
        let mut result = (0, 1, 0, 0);
        match self.child {
            NodeLimited::Inner(ref inner) => {
                let tmp = inner.borrow().nnodes();
                result.0 += tmp.0;
                result.1 += tmp.1;
                result.2 += tmp.2;
                result.3 += tmp.3;
            },
            NodeLimited::Child(_) => {
                result.2 += 1;
            },
        }
        result
    }
}


pub struct RdxTree<T> where T: Clone + Rdx {
    root: Node<T>,
}


impl<T> RdxTree<T> where T: Clone + Rdx {
    pub fn new() -> RdxTree<T> {
        let rounds = <T as Rdx>::cfg_nrounds();
        let buckets = <T as Rdx>::cfg_nbuckets();
        RdxTree {
            root: Node::Inner(
                Rc::new(RefCell::new(NodeInner::<T>::new(rounds, buckets)))
            ),
        }
    }

    pub fn insert(&mut self, x: T) {
        match self.root {
            Node::Inner(ref mut inner) => {
                inner.borrow_mut().insert(x);
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
                iters.push((inner.clone(), 0));
            },
            _ => unreachable!(),
        }
        RdxTreeIter {
            iters: iters,
            phantom: PhantomData,
        }
    }

    pub fn nnodes(&self) -> (usize, usize, usize, usize) {
        match self.root {
            Node::Inner(ref inner) => {
                inner.borrow().nnodes()
            },
            _ => {
                unreachable!()
            }
        }
    }
}


pub struct RdxTreeIter<'a, T> where T: Clone + Rdx + 'a {
    iters: Vec<(Rc<RefCell<NodeInner<T>>>, usize)>,  // do not work with iterators directly since we need a checked but dynamic borrow
    phantom: PhantomData<&'a RdxTree<T>>,            // keep tree borrow intact
}


impl<'a, T> Iterator for RdxTreeIter<'a, T> where T: Clone + Rdx + 'a {
    type Item = T;  // XXX: do not copy!

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Option<T> = None;

        while self.iters.len() > 0 && result.is_none() {
            let mut push: Option<Rc<RefCell<NodeInner<T>>>> = None;
            let mut pop = false;

            if let Some(pair) = self.iters.last_mut() {
                let &mut (ref rc, ref mut i) = pair;
                let borrow = rc.borrow();
                if *i >= borrow.children.len() {
                    pop = true;
                } else {
                    match borrow.children[*i] {
                        Node::Free => {},
                        Node::Child(ref x) => {
                            result = Some(x.clone());
                        },
                        Node::Inner(ref inner) => {
                            push = Some(inner.clone());
                        },
                        Node::Pruned(ref pruned) => {
                            match pruned.borrow().child {
                                NodeLimited::Child(ref x) => {
                                    result = Some(x.clone());
                                },
                                NodeLimited::Inner(ref inner) => {
                                    push = Some(inner.clone());
                                },
                            }
                        },
                    }

                    *i += 1;
                }
            } else {
                unreachable!();
            }

            if pop {
                self.iters.pop();
            } else if let Some(next) = push {
                self.iters.push((next, 0));
            }
        }

        result
    }
}


fn print_node<T>(node: &Node<T>, depth: usize) where T: Clone + fmt::Display + Rdx {
    let prefix: String = (0..depth).map(|_| ' ').collect();
    match *node {
        Node::Inner(ref inner) => {
            for (i, c) in inner.borrow().children.iter().enumerate() {
                println!("{}{}:", prefix, i);
                print_node(c, depth + 1);
            }
        },
        Node::Pruned(ref pruned) => {
            let borrowed = pruned.borrow();
            println!("{}P: [{:?}]", prefix, borrowed.buckets);
            let c = (&(borrowed.child)).into();
            print_node(&c, depth + borrowed.buckets.len());
        },
        Node::Child(ref x) => {
            println!("{}=> {}", prefix, x);
        },
        Node::Free => {
            println!("{}X", prefix);
        },
    }
}


fn print_tree<T>(tree: &RdxTree<T>) where T: Clone + fmt::Display + Rdx {
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
    let is: Vec<u32> = tree.iter().collect();
    assert_eq!(should, is);
    assert_eq!(tree.nnodes(), (4, 3, 5, 56));
}
