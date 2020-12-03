// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::borrow::Borrow;
use core::cmp::Ordering;

use canonical::{Canon, Store};
use canonical_derive::Canon;

use crate::annotation::Annotation;
use crate::compound::{Child, Compound};

/// Annotation to keep track of the largest element of a collection
#[derive(Canon, PartialEq, Debug, Clone, Copy)]
pub enum Max<K> {
    /// Identity of max, everything else is larger
    NegativeInfinity,
    /// Actual max value
    Maximum(K),
}

impl<K> PartialEq<K> for Max<K>
where
    K: PartialOrd,
{
    fn eq(&self, rhs: &K) -> bool {
        match (self, rhs) {
            (Max::Maximum(k), rhs) => k == rhs,
            _ => false,
        }
    }
}

impl<K> PartialOrd<K> for Max<K>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &K) -> Option<Ordering> {
        match self {
            Max::NegativeInfinity => Some(Ordering::Less),
            Max::Maximum(m) => m.partial_cmp(other),
        }
    }
}

impl<K> PartialOrd for Max<K>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &Max<K>) -> Option<Ordering> {
        // Prevent ordering inconsistency for cmp between two negative infinity
        if self == other {
            return Some(Ordering::Equal);
        }

        match other {
            Max::NegativeInfinity => Some(Ordering::Greater),
            Max::Maximum(other) => self.partial_cmp(other),
        }
    }
}

impl<K, C, S> Annotation<C, S> for Max<K>
where
    K: PartialOrd + Clone,
    C: Compound<S>,
    C::Leaf: Borrow<K>,
    S: Store,
{
    fn identity() -> Self {
        Max::NegativeInfinity
    }

    fn from_leaf(leaf: &C::Leaf) -> Self {
        Max::Maximum(leaf.borrow().clone())
    }

    fn from_node(node: &C) -> Self
    where
        Self: for<'any> From<&'any C::Leaf>,
    {
        let mut max = Max::NegativeInfinity;
        for i in 0.. {
            match node.child(i) {
                Child::EndOfNode => return max,
                Child::Leaf(l) => {
                    let new = l.into();
                    if new > max {
                        max = new
                    }
                }
                Child::Node(n) => {
                    let new = n.annotation().borrow();
                    if new > &max {
                        max = new.clone()
                    }
                }
            }
        }
        unreachable!()
    }
}

impl<'a, T, K> From<&'a T> for Max<K>
where
    T: Borrow<K>,
    K: Clone + PartialOrd,
{
    fn from(t: &T) -> Self {
        Max::Maximum(t.borrow().clone())
    }
}
