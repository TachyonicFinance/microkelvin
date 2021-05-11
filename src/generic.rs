use alloc::vec::Vec;

use canonical::Canon;
use canonical_derive::Canon;

use crate::link::Link;
use crate::{Annotation, Child, ChildMut, Compound};

#[derive(Clone, Canon, Debug)]
pub struct GenericAnnotation(Vec<u8>);

#[derive(Clone, Canon, Debug)]
pub struct GenericLeaf(Vec<u8>);

#[derive(Clone, Canon, Debug)]
pub enum GenericChild {
    Empty,
    Leaf(GenericLeaf),
    Link(Link<GenericTree, GenericAnnotation>),
}

#[derive(Default, Clone, Canon, Debug)]
pub struct GenericTree(Vec<GenericChild>);

impl GenericTree {
    pub fn new<C, A>(_c: &C) -> Self
    where
        C: Compound<A>,
        A: Annotation<C::Leaf>,
    {
        todo!()
    }

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
                GenericChild::Link(link) => Child::Node(link),
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
                GenericChild::Link(link) => ChildMut::Node(link),
            },
            None => ChildMut::EndOfNode,
        }
    }
}

impl Canon for Link<GenericTree, GenericAnnotation> {
    fn encode(&self, _sink: &mut canonical::Sink) {
        todo!()
    }

    fn decode(
        _source: &mut canonical::Source,
    ) -> Result<Self, canonical::CanonError> {
        todo!()
    }

    fn encoded_len(&self) -> usize {
        todo!()
    }
}
