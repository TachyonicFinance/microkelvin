use alloc::vec::Vec;

use canonical::{Canon, EncodeToVec};
use canonical_derive::Canon;

use crate::link::Link;
use crate::{Annotation, Child, ChildMut, Compound};

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
    pub fn new<C, A>(c: &C) -> Self
    where
        C: Compound<A>,
        C::Leaf: Canon,
        A: Annotation<C::Leaf> + Canon,
    {
        let mut generic = GenericTree::default();
        for i in 0.. {
            match c.child(i) {
                Child::Empty => generic.push(GenericChild::Empty),
                Child::Leaf(leaf) => {
                    generic.push(GenericChild::Leaf(GenericLeaf::new(leaf)))
                }
                Child::Node(link) => {
                    generic.push(GenericChild::Link(link.generic()))
                }
                Child::EndOfNode => break,
            }
        }
        generic
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
