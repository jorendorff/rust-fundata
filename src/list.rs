//! 2.1 Lists

use std::rc::Rc;
use std::iter::FromIterator;
use traits::Stack;

pub enum List<T> {
    Nil,
    Cons(Rc<(T, List<T>)>)
}

use self::List::*;

impl<V> Clone for List<V> {
    // `#[derive(Clone)]` doesn't work on List because it (not-very-smartly)
    // drives `impl <V: Clone> Clone for List<V>` instead of the more
    // general implementation we want.
    fn clone(&self) -> List<V> {
        match *self {
            Nil => Nil,
            Cons(ref rc) => Cons((*rc).clone())
        }
    }
}

impl<V> Stack for List<V> {
    type Item = V;

    fn empty() -> List<V> { Nil }

    fn is_empty(&self) -> bool {
        match *self {
            Nil => true,
            _ => false
        }
    }

    fn cons(head: V, tail: List<V>) -> List<V> {
        Cons(Rc::new((head, tail)))
    }

    fn split(&self) -> Option<(&V, &List<V>)> {
        match *self {
            Nil => None,
            Cons(ref rc) => {
                let r = &**rc;
                Some((&r.0, &r.1))
            }
        }
    }
}

pub struct ListIterator<V>(List<V>);

impl<V> List<V> {
    pub fn iter(&self) -> ListIterator<V> {
        ListIterator(self.clone())
    }
}

impl<V: Clone> IntoIterator for List<V> {
    type Item = V;
    type IntoIter = ListIterator<V>;
    fn into_iter(self) -> ListIterator<V> {
        ListIterator(self)
    }
}

impl<V: Clone> Iterator for ListIterator<V> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        self.0.pop()
    }
}

impl<V> FromIterator<V> for List<V> {
    fn from_iter<Iterable: IntoIterator<Item=V>>(iterator: Iterable) -> List<V>
    {
        let mut result = List::empty();
        let v: Vec<_> = iterator.into_iter().collect();
        for v in v.into_iter().rev() {
            result.push(v);
        }
        result
    }
}

impl<V> List<V> {
    pub fn length(&self) -> usize {
        let mut p = self.clone();
        let mut len = 0;
        loop {
            let next = match p {
                Nil => return len,
                Cons(ref rc) => rc.1.clone()
            };
            p = next;
            len += 1;
        }
    }
}

impl<V: Clone> List<V> {
    /// Split a list into its head and tail, or `None` if the list is empty.
    ///
    /// This requires the item type to be cloneable because 
    pub fn split_into(&self) -> Option<(V, List<V>)> {
        match *self {
            Nil => None,
            Cons(ref rc) => Some((**rc).clone())
        }
    }
}

/// Reverse a list.
///
/// This copies the entire list and all the items.
///
pub fn reverse<V: Clone>(s: List<V>) -> List<V> {
    let mut result = Nil;
    let mut current = &s;
    loop {
        match current.split() {
            None => {
                return result;
            },
            Some((first, rest)) => {
                result.push((*first).clone());
                current = rest;
            }
        }
    }
}

pub fn concat<S: Stack>(a: &S, b: S) -> S
    where S::Item: Clone
{
    match a.split() {
        None => b,
        Some((first, rest)) => S::cons(first.clone(), concat(rest, b))
    }
}

pub fn suffixes<S: Stack>(a: &S) -> List<S>
    where S: Clone
{
    match a.tail() {
        None => List::cons(a.clone(), List::empty()),
        Some(rest) =>
            List::cons(a.clone(), suffixes(rest))
    }
}

