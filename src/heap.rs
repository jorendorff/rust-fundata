//! 3.1 Leftist heaps

use std::cmp::Ordering;
use std::rc::Rc;
use traits::Heap;

struct HeapNode<V> {
    rank: usize,
    value: V,
    left: LeftistHeap<V>,
    right: LeftistHeap<V>
}

#[derive(Clone)]
enum HeapImpl<V> {
    Empty,
    NonEmpty(Rc<HeapNode<V>>)
}

use self::HeapImpl::*;

/// Simple persistent heap implementation. For documentation, see the `Heap` trait.
#[derive(Clone)]
pub struct LeftistHeap<V>(HeapImpl<V>);

impl<V> HeapImpl<V> {
    fn rank(&self) -> usize {
        match *self {
            Empty => 0,
            NonEmpty(ref rc) => (*rc).rank
        }
    }
}

fn make_heap<V: Clone>(x: V, a: LeftistHeap<V>, b: LeftistHeap<V>) -> LeftistHeap<V> {
    let LeftistHeap(ai) = a;
    let LeftistHeap(bi) = b;
    let ra = ai.rank();
    let rb = bi.rank();
    if ra >= rb {
        LeftistHeap(NonEmpty(Rc::new(HeapNode {
            rank: rb + 1,
            value: x,
            left: LeftistHeap(ai),
            right: LeftistHeap(bi)
        })))
    } else {
        LeftistHeap(NonEmpty(Rc::new(HeapNode {
            rank: ra + 1,
            value: x,
            left: LeftistHeap(bi),
            right: LeftistHeap(ai)
        })))
    }
}

impl<V: Clone + Ord> Heap for LeftistHeap<V> {
    type Item = V;

    fn empty() -> LeftistHeap<V> { LeftistHeap(Empty) }

    fn is_empty(&self) -> bool {
        match *self {
            LeftistHeap(Empty) => true,
            _ => false
        }
    }

    fn merge(h1: LeftistHeap<V>, h2: LeftistHeap<V>) -> LeftistHeap<V> {
        match (h1, h2) {
            (LeftistHeap(Empty), h) => h,
            (h, LeftistHeap(Empty)) => h,
            (LeftistHeap(NonEmpty(n1)), LeftistHeap(NonEmpty(n2))) => {
                if n1.value.cmp(&n2.value) == Ordering::Greater {
                    make_heap(n2.value.clone(),
                              n2.left.clone(),
                              LeftistHeap::merge(LeftistHeap(NonEmpty(n1)),
                                                 n2.right.clone()))
                } else {
                    make_heap(n1.value.clone(),
                              n1.left.clone(),
                              LeftistHeap::merge(n1.right.clone(),
                                                 LeftistHeap(NonEmpty(n2))))
                }
            }
        }
    }

    fn insert(&self, value: V) -> LeftistHeap<V> {
        LeftistHeap::merge(self.clone(), LeftistHeap(NonEmpty(Rc::new(HeapNode {
            rank: 1,
            value: value,
            left: LeftistHeap(Empty),
            right: LeftistHeap(Empty)
        }))))
    }

    fn min(&self) -> Option<&V> {
        match *self {
            LeftistHeap(Empty) => None,
            LeftistHeap(NonEmpty(ref n)) => Some(&n.value)
        }
    }

    fn without_min(&self) -> LeftistHeap<V> {
        match *self {
            LeftistHeap(Empty) => LeftistHeap(Empty),
            LeftistHeap(NonEmpty(ref n)) => LeftistHeap::merge(n.left.clone(), n.right.clone())
        }
    }
}
