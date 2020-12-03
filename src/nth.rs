// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::borrow::Borrow;

use canonical::Store;

use crate::branch::{Branch, Step, Walk};
use crate::branch_mut::{BranchMut, StepMut, WalkMut};

use crate::compound::Compound;
use crate::impls::cardinality::Cardinality;

/// Find the nth element of any collection satisfying the given annotation
/// constraints
pub trait Nth<'a, S>
where
    Self: Compound<S> + Sized,
    S: Store,
{
    /// Construct a `Branch` pointing to the `nth` element, if any
    fn nth<A, const N: usize>(
        &'a self,
        n: u64,
    ) -> Result<Option<Branch<'a, Self, A, S, N>>, S::Error>;

    /// Construct a `BranchMut` pointing to the `nth` element, if any
    fn nth_mut<A, const N: usize>(
        &'a mut self,
        n: u64,
    ) -> Result<Option<BranchMut<'a, Self, A, S, N>>, S::Error>;
}

impl<'a, C, S> Nth<'a, S> for C
where
    C: Compound<S>,
    S: Store,
{
    fn nth<A, const N: usize>(
        &'a self,
        mut index: u64,
    ) -> Result<Option<Branch<'a, Self, A, S, N>>, S::Error> {
        Branch::walk(self, |f| match f {
            Walk::Leaf(l) => {
                if index == 0 {
                    Step::Found(l)
                } else {
                    index -= 1;
                    Step::Next
                }
            }
            Walk::Node(n) => {
                let &Cardinality(card) = n.annotation().borrow();
                if card <= index {
                    index -= card;
                    Step::Next
                } else {
                    Step::Into(n)
                }
            }
        })
    }

    fn nth_mut<A, const N: usize>(
        &'a mut self,
        mut index: u64,
    ) -> Result<Option<BranchMut<'a, Self, A, S, N>>, S::Error> {
        BranchMut::walk(self, |f| match f {
            WalkMut::Leaf(l) => {
                if index == 0 {
                    StepMut::Found(l)
                } else {
                    index -= 1;
                    StepMut::Next
                }
            }
            WalkMut::Node(n) => {
                let &Cardinality(card) = n.annotation().borrow();
                if card <= index {
                    index -= card;
                    StepMut::Next
                } else {
                    StepMut::Into(n)
                }
            }
        })
    }
}
