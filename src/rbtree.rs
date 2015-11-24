/* 3.3 Red-Black Trees */

use std::rc::Rc;
use traits::Set;
use std::cmp::Ordering::*;

#[derive(PartialEq, Clone, Copy)]
enum Color { Red, Black }

struct RBTreeNode<V> {
    color: Color,
    value: V,
    left: RBTree<V>,
    right: RBTree<V>
}

// Implementation note: an RBTreeImpl is either empty or a pointer to a tree
// node. Each tree node contains a color. It's a fairly common operation to
// copy a tree node in order to change its color from red to black; this
// wouldn't be necessary if we stored the color bit in the RBTreeImpl, and
// there are plenty of spare bits here. But I don't think Rust is miserly
// enough to use those spare bits; I think it would bloat.
#[derive(Clone)]
enum RBTreeImpl<V> {
    RBEmpty,
    RBNonEmpty(Rc<RBTreeNode<V>>)
}

/// Red-black balanced binary trees. Use the `Set` methods.
#[derive(Clone)]
pub struct RBTree<V>(RBTreeImpl<V>);

use self::Color::*;
use self::RBTreeImpl::*;

fn black<V: Clone>(left: &RBTree<V>, value: &V, right: &RBTree<V>) -> RBTree<V> {
    RBTree(RBNonEmpty(Rc::new(RBTreeNode {
        color: Black,
        value: value.clone(),
        left: left.clone(),
        right: right.clone()
    })))
}

fn build_rotated_nodes<V: Clone>(a: &RBTree<V>,
                                 x: &V,
                                 b: &RBTree<V>,
                                 y: &V,
                                 c: &RBTree<V>,
                                 z: &V,
                                 d: &RBTree<V>)
                                 -> Rc<RBTreeNode<V>>
{
    Rc::new(RBTreeNode {
        color: Red,
        value: y.clone(),
        left: black(a, x, b),
        right: black(c, z, d)
    })
}

fn balance<V: Clone>(color: Color, left_tree: RBTree<V>, value: V, right_tree: RBTree<V>)
                     -> Rc<RBTreeNode<V>>
{
    if color == Black {
        match left_tree.0 {
            RBEmpty => (),
            RBNonEmpty(ref l_rc) => {
                let l = &**l_rc;
                if l.color == Red {
                    match l.left.0 {
                        RBNonEmpty(ref ll) if ll.color == Red => {
                            return build_rotated_nodes(
                                &ll.left, &ll.value, &ll.right, &l.value,
                                &l.right, &value, &right_tree);
                        },
                        _ => ()
                    }
                    match l.right.0 {
                        RBNonEmpty(ref lr) if lr.color == Red => {
                            return build_rotated_nodes(
                                &l.left, &l.value, &lr.left, &lr.value,
                                &lr.right, &value, &right_tree);
                        },
                        _ => (),
                    }
                }
            }
        }
        match right_tree.0 {
            RBEmpty => (),
            RBNonEmpty(ref r_rc) => {
                let r = &**r_rc;
                if r.color == Red {
                    match r.left.0 {
                        RBNonEmpty(ref rl) if rl.color == Red => {
                            return build_rotated_nodes(
                                &left_tree, &value, &rl.left, &rl.value,
                                &rl.right, &r.value, &r.right);
                        },
                        _ => ()
                    }
                    match r.right.0 {
                        RBNonEmpty(ref rr) if rr.color == Red => {
                            return build_rotated_nodes(
                                &left_tree, &value, &r.left, &r.value,
                                &rr.left, &rr.value, &rr.right);
                        },
                        _ => ()
                    }
                }
            }
        }
    }
    return Rc::new(RBTreeNode {
        color: color,
        value: value,
        left: left_tree,
        right: right_tree
    });
}

impl<V: Clone> RBTree<V> {
    fn copy_to_vec(&self, out: &mut Vec<V>) {
        match self.0 {
            RBEmpty => (),
            RBNonEmpty(ref rc) => {
                let r = &**rc;
                r.left.copy_to_vec(out);
                out.push(r.value.clone());
                r.right.copy_to_vec(out);
            }
        }
    }
}

impl<V: Clone> IntoIterator for RBTree<V> {
    type Item = V;
    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        let mut v = vec![];
        self.copy_to_vec(&mut v);
        v.into_iter()
    }
}

fn ins<V: Clone + Ord>(tree: &RBTree<V>, value: V) -> Rc<RBTreeNode<V>> {
    match tree.0 {
        RBEmpty => Rc::new(RBTreeNode {
            color: Red,
            value: value,
            left: RBTree(RBEmpty),
            right: RBTree(RBEmpty)
        }),
        RBNonEmpty(ref rc) => {
            match value.cmp(&rc.value) {
                Less => balance(
                    rc.color,
                    RBTree(RBNonEmpty(ins(&rc.left, value))),
                    rc.value.clone(),
                    rc.right.clone()),
                Greater => balance(
                    rc.color,
                    rc.left.clone(),
                    rc.value.clone(),
                    RBTree(RBNonEmpty(ins(&rc.right, value)))),
                Equal => (*rc).clone()
            }
        }
    }
}

impl<V: Clone + Ord> Set for RBTree<V> {
    fn empty() -> RBTree<V> { RBTree(RBEmpty) }

    fn plus(&self, value: V) -> RBTree<V> {
        let rc = ins(self, value);
        if rc.color == Red {
            black(&rc.left, &rc.value, &rc.right)
        } else {
            RBTree(RBNonEmpty(rc))
        }
    }
    
    fn contains(&self, value: &V) -> bool {
        match self.0 {
            RBEmpty => false,
            RBNonEmpty(ref rc) => {
                let r = &**rc;
                match value.cmp(&r.value) {
                    Less => r.left.contains(value),
                    Greater => r.right.contains(value),
                    Equal => true
                }
            }
        }
    }
}

