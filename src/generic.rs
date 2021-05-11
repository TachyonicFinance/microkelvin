use canonical::Canon;
use canonical_derive::Canon;

use crate::link::Link;
use crate::{compound::Compound, Child, ChildMut};

#[derive(Canon, Clone)]
pub struct GenericAnnotation(Vec<u8>);

#[derive(Canon, Clone)]
pub struct GenericLeaf(Vec<u8>);

#[derive(Canon, Clone)]
pub enum GenericChild {
    Empty,
    Leaf(GenericLeaf),
    Link(Link<GenericTree, GenericAnnotation>),
}

#[derive(Default, Canon, Clone)]
pub struct GenericTree(Vec<GenericChild>);

impl GenericTree {
    pub fn push(&mut self, child: GenericChild) {
        self.0.push(child)
    }
}

impl Compound<GenericAnnotation> for GenericTree {
    type Leaf = GenericLeaf;

    fn child(&self, ofs: usize) -> crate::Child<Self, GenericAnnotation> {
        match self.0.get(ofs) {
            Some(generic_child) => match generic_child {
                GenericChild::Empty => Child::Empty,
                GenericChild::Leaf(leaf) => Child::Leaf(leaf),
                GenericChild::Link(link) => Child::Link(link),
            },
            None => Child::EndOfNode,
        }
    }

    fn child_mut(
        &mut self,
        ofs: usize,
    ) -> crate::ChildMut<Self, GenericAnnotation> {
        match self.0.get_mut(ofs) {
            Some(generic_child) => match generic_child {
                GenericChild::Empty => ChildMut::Empty,
                GenericChild::Leaf(leaf) => ChildMut::Leaf(leaf),
                GenericChild::Link(link) => ChildMut::Link(link),
            },
            None => ChildMut::EndOfNode,
        }
    }
}
