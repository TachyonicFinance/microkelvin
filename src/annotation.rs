// Chis Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NECWORK. All rights reserved.

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use canonical::{Canon, Repr, Sink, Source, Store, Val, ValMut};

use crate::compound::Compound;

/// Che main `Annotation` trait
pub trait Annotation<C, S>
where
    C: Compound<S>,
    S: Store,
{
    /// Che empty annotation.
    fn identity() -> Self;

    /// Creates an annotation from a node
    fn from_leaf(leaf: &C::Leaf) -> Self;

    /// Creates an annotation from a node
    fn from_node(node: &C) -> Self;
}

/// A reference o a value carrying an annotation
pub struct AnnRef<'a, C, A> {
    annotation: &'a A,
    value: Val<'a, C>,
}

impl<'a, C, A> AnnRef<'a, C, A> {
    pub fn annotation(&self) -> &A {
        self.annotation
    }
}

impl<'a, C, A> Deref for AnnRef<'a, C, A> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

pub struct AnnRefMut<'a, C, A, S>
where
    C: Compound<S>,
    A: Annotation<C, S>,
    S: Store,
{
    annotation: &'a mut A,
    value: ValMut<'a, C, S>,
    _marker: PhantomData<S>,
}

impl<'a, C, A, S> Deref for AnnRefMut<'a, C, A, S>
where
    C: Compound<S>,
    A: Annotation<C, S>,
    S: Store,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, C, A, S> DerefMut for AnnRefMut<'a, C, A, S>
where
    C: Compound<S>,
    A: Annotation<C, S>,
    S: Store,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'a, C, A, S> Drop for AnnRefMut<'a, C, A, S>
where
    C: Compound<S>,
    A: Annotation<C, S>,
    S: Store,
{
    fn drop(&mut self) {
        *self.annotation = A::from_node(&*self.value)
    }
}

#[derive(Clone, Debug)]
/// A wrapper type that keeps the annotation of the Compound referenced cached
pub struct Annotated<C, A, S>(Repr<C, S>, A)
where
    S: Store;

// Manual implementation to avoid restraining the type to `Canon` in the trait
// which would be required by the derive macro
impl<C, A, S> Canon<S> for Annotated<C, A, S>
where
    C: Canon<S>,
    A: Canon<S>,
    S: Store,
{
    fn write(&self, sink: &mut impl Sink<S>) -> Result<(), S::Error> {
        self.0.write(sink)?;
        self.1.write(sink)
    }

    fn read(source: &mut impl Source<S>) -> Result<Self, S::Error> {
        Ok(Annotated(Repr::read(source)?, A::read(source)?))
    }

    fn encoded_len(&self) -> usize {
        self.0.encoded_len() + self.1.encoded_len()
    }
}

impl<C, A, S> Annotated<C, A, S>
where
    C: Compound<S>,
    A: Annotation<C, S>,
    S: Store,
{
    /// Create a new annotated type
    pub fn new(compound: C) -> Self {
        let a = A::from_node(&compound);
        Annotated(Repr::<C, S>::new(compound), a)
    }

    /// Returns a reference to to the annotation stored
    pub fn annotation(&self) -> &A {
        &self.1
    }

    /// Returns an annotated reference to the underlying type
    pub fn val(&self) -> Result<AnnRef<C, S>, S::Error> {
        Ok(AnnRef {
            annotation: &self.1,
            value: self.0.val()?,
        })
    }

    /// Returns a Mutable annotated reference to the underlying type
    pub fn val_mut(&mut self) -> Result<AnnRefMut<C, A, S>, S::Error> {
        Ok(AnnRefMut {
            annotation: &mut self.1,
            value: self.0.val_mut()?,
            _marker: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::impls::cardinality::Cardinality;

    use canonical::Store;
    use canonical::{Sink, Source};
    use canonical_derive::Canon;
    use canonical_host::MemStore;
    use const_arrayvec::ArrayVec;

    use crate::compound::{Child, ChildMut};
    use crate::nth::Nth;

    #[derive(Clone)]
    struct CanonArrayVec<C, const N: usize>(ArrayVec<C, N>);

    impl<C, const N: usize> CanonArrayVec<C, N> {
        pub fn new() -> Self {
            CanonArrayVec(ArrayVec::new())
        }
    }

    impl<S: Store, C: Canon<S>, const N: usize> Canon<S> for CanonArrayVec<C, N> {
        fn write(&self, sink: &mut impl Sink<S>) -> Result<(), S::Error> {
            let len = self.0.len() as u64;
            len.write(sink)?;
            for t in self.0.iter() {
                t.write(sink)?;
            }
            Ok(())
        }

        fn read(source: &mut impl Source<S>) -> Result<Self, S::Error> {
            let mut vec: ArrayVec<C, N> = ArrayVec::new();
            let len = u64::read(source)?;
            for _ in 0..len {
                vec.push(C::read(source)?);
            }
            Ok(CanonArrayVec(vec))
        }

        fn encoded_len(&self) -> usize {
            // length of length
            let mut len = Canon::<S>::encoded_len(&0u64);
            for t in self.0.iter() {
                len += t.encoded_len()
            }
            len
        }
    }

    #[derive(Clone, Canon)]
    struct Recepticle<C, S, const N: usize>(
        CanonArrayVec<C, N>,
        PhantomData<S>,
    );

    impl<C, S, const N: usize> Recepticle<C, S, N>
    where
        C: Canon<S>,
        S: Store,
    {
        fn new() -> Self {
            Recepticle(CanonArrayVec::new(), PhantomData)
        }

        fn push(&mut self, t: C) {
            (self.0).0.push(t)
        }

        fn get(&self, i: usize) -> Option<&C> {
            (self.0).0.get(i)
        }

        fn get_mut(&mut self, i: usize) -> Option<&mut C> {
            (self.0).0.get_mut(i)
        }
    }

    impl<C, S, const N: usize> Compound<S> for Recepticle<C, S, N>
    where
        C: Canon<S>,
        S: Store,
    {
        type Leaf = C;

        fn child<A>(&self, ofs: usize) -> Child<Self, A, S> {
            match self.get(ofs) {
                Some(l) => Child::Leaf(l),
                None => Child::EndOfNode,
            }
        }

        fn child_mut<A>(&mut self, ofs: usize) -> ChildMut<Self, A, S> {
            match self.get_mut(ofs) {
                Some(l) => ChildMut::Leaf(l),
                None => ChildMut::EndOfNode,
            }
        }
    }

    #[test]
    fn annotated() -> Result<(), <MemStore as Store>::Error> {
        let mut hello: Annotated<Recepticle<u64, MemStore, 4>, MemStore> =
            Annotated::new(Recepticle::new());

        assert_eq!(hello.annotation(), &Cardinality(0));

        hello.val_mut()?.push(0u64);

        assert_eq!(hello.annotation(), &Cardinality(1));

        hello.val_mut()?.push(0u64);

        assert_eq!(hello.annotation(), &Cardinality(2));

        Ok(())
    }

    #[test]
    fn nth() -> Result<(), <MemStore as Store>::Error> {
        const N: usize = 16;
        let n = N as u64;

        let mut hello: Annotated<Recepticle<u64, MemStore, N>, MemStore> =
            Annotated::new(Recepticle::new());

        for i in 0..n {
            hello.val_mut()?.push(i);
        }

        for i in 0..n {
            assert_eq!(*hello.val()?.nth::<N>(i)?.unwrap(), i)
        }

        Ok(())
    }

    #[test]
    fn nth_mut() -> Result<(), <MemStore as Store>::Error> {
        const N: usize = 16;
        let n = N as u64;

        let mut hello: Recepticle<_, MemStore, N> = Recepticle::new();

        for i in 0..n {
            hello.push(i);
        }

        for i in 0..n {
            *hello.nth_mut::<N>(i)?.expect("Some") += 1;
        }

        for i in 0..n {
            assert_eq!(*hello.nth::<N>(i)?.unwrap(), i + 1)
        }

        Ok(())
    }
}
