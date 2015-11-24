// queue.rs - What you sing before "T, U, V"

use std::mem::swap;
use traits::{Queue, Stack};
use list::List;
use list::List::Nil;
use list::reverse;

/// A persistent queue implemented as a pair of linked lists.
pub struct BatchedQueue<T> {
    // We have an invariant that if front is empty, then back is empty.
    // Or equivalently: it's never true that the front is empty and the back isn't.
    front: List<T>,
    back: List<T>
}

// `derive(Clone)` is not smart enough to derive this instance, so we have to
// write it out. (Rust instead derives the more restricted `impl<T: Clone> Clone
// for BatchedQueue<T>`.)
impl<T> Clone for BatchedQueue<T> {
    fn clone(&self) -> BatchedQueue<T> {
        BatchedQueue {
            front: self.front.clone(),
            back: self.back.clone()
        }
    }
}

impl<T: Clone> BatchedQueue<T> {
    // Build a queue from components, moving items from back to front if needed
    // to preserve the invariant.
    fn build(front: List<T>, back: List<T>) -> BatchedQueue<T> {
        if front.is_empty() {
            BatchedQueue {
                front: reverse(back),
                back: Nil
            }
        } else {
            BatchedQueue {
                front: front,
                back: back
            }
        }
    }
}

impl<T: Clone> Queue for BatchedQueue<T> {
    type Item = T;

    /// Return an empty BatchedQueue.
    fn empty() -> BatchedQueue<T> {
        BatchedQueue { front: Nil, back: Nil }
    }

    /// Return true if there are no items in this queue.
    fn is_empty(&self) -> bool {
        // Because we have the invariant "if `front` is empty, then `back` is
        // empty", we only need to check `front`.
        self.front.is_empty()
    }

    /// Add a queue and an item.
    ///
    /// This returns a new queue with all the items in `queue`, plus a new item
    /// `value` added at the back.
    ///
    /// This runs in constant time and space. (It does not make a copy of
    /// the items in `queue`.)
    ///
    fn snoc(queue: BatchedQueue<T>, value: T) -> BatchedQueue<T> {
        if queue.is_empty() {
            // Separate implementation in order to maintain the invariant.
            BatchedQueue {
                front: List::cons(value, Nil),
                back: Nil
            }
        } else {
            let BatchedQueue { front, back } = queue;
            BatchedQueue {
                front: front,
                back: List::cons(value, back)
            }
        }
    }

    /// Split this queue into two parts: the item at the front and another
    /// queue containing everything else. If the queue is empty, this returns
    /// None.
    ///
    /// This runs in amortized constant time and space. That is, occasionally
    /// this method takes time and space proportional to the size of the queue,
    /// but over many `split` calls, the average time and space used is a
    /// low amount that doesn't increase as the size of the queue increases.
    ///
    fn split(&self) -> Option<(&T, BatchedQueue<T>)> {
        match self.front.split() {
            None => None,
            Some((first, rest)) =>
                Some((first, BatchedQueue::build((*rest).clone(), self.back.clone())))
        }
    }

    /// Get the front item.
    ///
    /// If the queue is empty, this returns `None`. Otherwise it returns `Some(r)`
    /// where `r` is a reference to the front item in this queue.
    ///
    /// This runs in constant time and space.
    ///
    fn head(&self) -> Option<&T> {
        self.front.head()
    }
}

impl<T: Clone> BatchedQueue<T> {
    /// Break this queue into two parts: the item at the front and another
    /// queue containing everything else. If the queue is empty, this returns
    /// None.
    ///
    /// This runs in amortized constant time and space. That is, occasionally
    /// this method takes time and space proportional to the size of the queue,
    /// but over many `split_into` calls, the average time and space used is a
    /// low amount that doesn't increase as the size of the queue increases.
    ///
    pub fn split_into(self) -> Option<(T, BatchedQueue<T>)> {
        match self.front.split_into() {
            None =>
                match reverse(self.back).split_into() {
                    None => None,
                    Some((first, rest)) =>
                        Some((first, BatchedQueue { front: rest, back: Nil }))
                },
            Some((first, rest)) =>
                Some((first, BatchedQueue { front: rest, back: self.back }))
        }
    }

    /* Mutators */

    /// Remove and return the item at the front of this queue.
    ///
    /// If the queue is empty, this returns None. Otherwise it returns Some(the
    /// removed item).
    ///
    /// This runs in amortized constant time and space, like `split_into`.
    ///
    pub fn pop_front(&mut self) -> Option<T> {
        let mut tmp = BatchedQueue::empty();
        swap(self, &mut tmp);
        match tmp.split_into() {
            None => None,
            Some((first, rest)) => {
                *self = rest;
                Some(first)
            }
        }
    }
}
