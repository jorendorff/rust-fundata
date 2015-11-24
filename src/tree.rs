//! Persistent set data structures.

use std::cmp::Ordering::*;
use std::iter::IntoIterator;
use std::rc::Rc;
use traits::Set;

struct TreeNode<V> {
    value: V,
    left: Tree<V>,
    right: Tree<V>
}

#[derive(Clone)]
enum TreeImpl<V> {
    Empty,
    NonEmpty(Rc<TreeNode<V>>)
}

/// An unbalanced tree implementation. Use the `Set` methods.
#[derive(Clone)]
pub struct Tree<V>(TreeImpl<V>);

fn cons_tree<V>(value: V, left: Tree<V>, right: Tree<V>) -> Tree<V> {
    Tree(NonEmpty(Rc::new(TreeNode {value: value, left: left, right: right})))
}

use self::TreeImpl::*;

impl<V: Ord + Clone> Set for Tree<V> {
    fn empty() -> Tree<V> { Tree(Empty) }

    fn plus(&self, v: V) -> Tree<V> {
        match self.0 {
            Empty => cons_tree(v, Tree(Empty), Tree(Empty)),
            NonEmpty(ref rc) => {
                let n = &**rc;
                match v.cmp(&n.value) {
                    Less => cons_tree(n.value.clone(), n.left.plus(v), n.right.clone()),
                    Greater => cons_tree(n.value.clone(), n.left.clone(), n.right.plus(v)),
                    Equal => self.clone()
                }
            }
        }
    }

    fn contains(&self, v: &V) -> bool {
        match self.0 {
            Empty => false,
            NonEmpty(ref rc) => match v.cmp(&(*rc).value) {
                Less => (*rc).left.contains(v),
                Greater => (*rc).right.contains(v),
                Equal => true
            }
        }
    }
}

impl<V: Clone> Tree<V> {
    fn copy_to_vec(&self, out: &mut Vec<V>) {
        match self.0 {
            Empty => (),
            NonEmpty(ref rc) => {
                let n = &**rc;
                n.left.copy_to_vec(out);
                out.push(n.value.clone());
                n.right.copy_to_vec(out);
            }
        }
    }
}

impl<V: Clone> IntoIterator for Tree<V> {
    type Item = V;
    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        let mut v = vec![];
        self.copy_to_vec(&mut v);
        v.into_iter()
    }
}


