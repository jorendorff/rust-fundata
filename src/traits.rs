//! Abstract descriptions of various kinds of persistent collections.

use std::mem::swap;

/// A Stack is a first-in-first-out collection.
///
/// Stacks store items in the order they are inserted. A stack supports adding
/// and removing items from one end.
///
pub trait Stack: Sized {
    type Item;
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(Self::Item, Self) -> Self;
    fn split(&self) -> Option<(&Self::Item, &Self)>;

    fn head<'a>(&'a self) -> Option<&'a Self::Item>
    {
        self.split().map(|(h, _)| h)
    }

    fn tail<'a>(&'a self) -> Option<&'a Self>
        where Self::Item: 'a
    {
        self.split().map(|(_, t)| t)
    }

    // Mutators. Rust's restrictions on mutation are so good we're tempted to
    // add a few niceties.

    fn push(&mut self, v: Self::Item) {
        let mut tmp = Self::empty();
        swap(self, &mut tmp);
        *self = Self::cons(v, tmp);
    }

    fn pop(&mut self) -> Option<Self::Item>
        where Self::Item: Clone, Self: Clone
    {
        let (first, rest) = match self.split() {
            None => return None,
            Some((h, t)) => (Some(h.clone()), t.clone())
        };
        *self = rest;
        first
    }
}

/// A queue is a first-in-first-out collection.
pub trait Queue: Clone {
    type Item;

    /// Create an empty queue.
    fn empty() -> Self;

    /// Return true if this queue is empty.
    fn is_empty(&self) -> bool {
        self.head().is_none()
    }

    /// Add an item to the back of a queue.
    fn snoc(Self, Self::Item) -> Self;

    /// Return a reference to the front item of a queue.
    ///
    /// If the queue is empty, this returns None.
    ///
    fn head(&self) -> Option<&Self::Item> {
        self.split().map(|pair| pair.0)
    }

    /// Return a copy of this queue, but omitting the front item.
    ///
    /// If the queue is empty, this returns None.
    ///
    fn tail(&self) -> Option<Self> {
        self.split().map(|pair| pair.1)
    }
    
    fn split(&self) -> Option<(&Self::Item, Self)>;
    
    /* Mutators */

    /// Add an item to this queue.
    ///
    /// This runs in constant time and space.
    ///
    fn push_back(&mut self, value: Self::Item) {
        *self = Self::snoc((*self).clone(), value);
    }
}

/// A deque is a queue that supports adding and removing items at either end.
pub trait Deque: Queue {
    fn cons(Self::Item, Self) -> Self;

    fn last(&self) -> Option<&Self::Item> {
        self.split_back().map(|pair| pair.1)
    }

    fn init(&self) -> Option<Self> {
        self.split_back().map(|pair| pair.0)
    }

    fn split_back(&self) -> Option<(Self, &Self::Item)>;
    
    fn push_front(&mut self, value: Self::Item) {
        *self = Self::cons(value, (*self).clone());
    }
}


/// A Set is a collection with efficient membership testing.
///
/// That is, you can use the `set.contains(value)` method to test whether a
/// set contains a given value.
///
pub trait Set: IntoIterator {
    /// Return an empty set.
    fn empty() -> Self;

    /// Return the union of `self` and the singleton set containing `value`.
    fn plus(&self, value: Self::Item) -> Self;

    /// Return true if the given value is in this set.
    fn contains(&self, value: &Self::Item) -> bool;

    /* Mutating operations. */

    /// Modify this set in-place by adding an item.
    fn add(&mut self, v: Self::Item)
        where Self: Sized
    {
        let mut tmp = Self::empty();
        swap(self, &mut tmp);
        *self = tmp.plus(v);
    }
}

/// A Heap is a collection that supports efficiently finding and removing the
/// minimum element.
///
pub trait Heap: Sized
    where Self::Item: Clone
{
    /// The type of value the heap contains.
    type Item;

    /// Return an empty heap value.
    fn empty() -> Self;

    /// Return true if this heap is empty.
    fn is_empty(&self) -> bool;

    /// Return a heap containing all the values in self, and also the given Item.
    fn insert(&self, Self::Item) -> Self;

    /// Create a new heap by combining two existing heaps.
    fn merge(Self, Self) -> Self;

    /// Return the minimum item in this heap, without removing it.
    /// If `self.is_empty()`, this returns `None`.
    fn min(&self) -> Option<&Self::Item>;

    /// Return a heap containing all the items in this heap except the minimum
    /// item. If `self.is_empty()`, this returns an empty heap.
    fn without_min(&self) -> Self;

    /* Mutating operations */

    /// Add an item to this heap.
    fn add(&mut self, value: Self::Item) {
        *self = self.insert(value);
    }
    
    /// Remove and return the minimum item of this heap. If `self.is_empty()`,
    /// this does nothing and returns `None`.
    fn pop(&mut self) -> Option<Self::Item> {
        let mut tmp = Self::empty();
        swap(self, &mut tmp);
        match tmp.min() {
            None => None,
            Some(r) => {
                *self = tmp.without_min();
                Some(r.clone())
            }
        }
    }
}
