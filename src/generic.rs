use alloc::vec::Vec;

use canonical::{Canon, EncodeToVec};
use canonical_derive::Canon;

use crate::link::Link;
use crate::{Child, ChildMut, Compound};

#[derive(Clone, Canon, Debug)]
pub struct GenericAnnotation(Vec<u8>);

#[derive(Clone, Canon, Debug)]
pub struct GenericLeaf(Vec<u8>);

impl GenericLeaf {
    pub(crate) fn new<C: Canon>(c: &C) -> Self {
        GenericLeaf(c.encode_to_vec())
    }
}

impl GenericAnnotation {
    pub(crate) fn new<A: Canon>(a: &A) -> Self {
        GenericAnnotation(a.encode_to_vec())
    }
}

#[derive(Clone, Canon, Debug)]
pub enum GenericChild {
    Empty,
    Leaf(GenericLeaf),
    Link(Link<GenericTree, GenericAnnotation>),
}

#[derive(Default, Clone, Canon, Debug)]
pub struct GenericTree(Vec<GenericChild>);

impl GenericTree {
    pub(crate) fn new() -> Self {
        GenericTree(vec![])
    }

    pub(crate) fn push_empty(&mut self) {
        self.0.push(GenericChild::Empty)
    }

    pub(crate) fn push_leaf<L>(&mut self, _leaf: &L) {
        todo!()
    }

    pub(crate) fn push_link<C, A>(&mut self, _link: &Link<C, A>) {
        todo!()
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
