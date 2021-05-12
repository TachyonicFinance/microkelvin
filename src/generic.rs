use alloc::vec::Vec;

use canonical::{Canon, CanonError, EncodeToVec, Id};
use canonical_derive::Canon;

use crate::link::Link;
use crate::{Annotation, Compound};

const TAG_EMPTY: u8 = 0;
const TAG_LEAF: u8 = 1;
const TAG_LINK: u8 = 2;

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

#[derive(Clone, Debug)]
pub enum GenericChild {
    Empty,
    Leaf(GenericLeaf),
    Link(Id, GenericAnnotation),
}

impl Canon for GenericChild {
    fn encode(&self, sink: &mut canonical::Sink) {
        match self {
            Self::Empty => TAG_EMPTY.encode(sink),
            Self::Leaf(leaf) => {
                TAG_LEAF.encode(sink);
                let leaf_len = leaf.encoded_len();
                assert!(leaf_len < u16::MAX as usize);
                (leaf_len as u16).encode(sink);
                leaf.encode(sink)
            }
            Self::Link(id, annotation) => {
                TAG_LINK.encode(sink);
                id.encode(sink);
                annotation.encode(sink);
            }
        }
    }

    fn decode(source: &mut canonical::Source) -> Result<Self, CanonError> {
        todo!()
    }

    fn encoded_len(&self) -> usize {
        let tag_len = 1;
        match self {
            Self::Empty => tag_len,
            Self::Leaf(leaf) => {
                let size_len = 2;
                tag_len + size_len + leaf.encoded_len()
            }
            Self::Link(id, anno) => {
                tag_len + id.encoded_len() + anno.encoded_len()
            }
        }
    }
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

    pub(crate) fn push_leaf<L: Canon>(&mut self, leaf: &L) {
        self.0.push(GenericChild::Leaf(GenericLeaf::new(leaf)))
    }

    pub(crate) fn push_link<C, A>(&mut self, link: &Link<C, A>)
    where
        C: Compound<A>,
        C::Leaf: Canon,
        A: Annotation<C::Leaf> + Canon,
    {
        let id = link.id();
        let anno = GenericAnnotation::new(&*link.annotation());
        self.0.push(GenericChild::Link(id, anno));
    }
}

// impl Canon for Link<GenericTree, GenericAnnotation> {
//     fn encode(&self, _sink: &mut canonical::Sink) {
//         todo!()
//     }

//     fn decode(
//         _source: &mut canonical::Source,
//     ) -> Result<Self, canonical::CanonError> {
//         todo!()
//     }

//     fn encoded_len(&self) -> usize {
//         self.id().encoded_len() + self.annotation().encoded_len()
//     }
// }
