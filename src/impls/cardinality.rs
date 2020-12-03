// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::borrow::Borrow;
use core::ops::AddAssign;

use canonical::{Canon, Store};
use canonical_derive::Canon;

use crate::annotation::Annotation;
use crate::compound::{Child, Compound};

/// Annotation to keep track of the cardinality,
/// i.e. the amount of elements of a collection

#[derive(Canon, PartialEq, Debug, Clone, Copy)]
pub struct Cardinality(pub(crate) u64);

impl AddAssign for Cardinality {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl Into<u64> for &Cardinality {
    fn into(self) -> u64 {
        self.0
    }
}

impl<C, S> Annotation<C, S> for Cardinality
where
    C: Compound<S>,
    S: Store,
{
    fn identity() -> Self {
        Cardinality(0)
    }

    fn from_leaf(leaf: &C::Leaf) -> Self {
        Cardinality(1)
    }

    fn from_node(node: &C) -> Self {
        let mut count = Cardinality(0);
        for i in 0.. {
            match node.child(i) {
                Child::EndOfNode => return count,
                Child::Leaf(l) => count += l.into(),
                Child::Node(n) => count += *n.annotation().borrow(),
            }
        }
        unreachable!()
    }
}
