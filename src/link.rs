use alloc::rc::Rc;
use core::cell::{RefCell, RefMut};
use core::mem;
use core::ops::{Deref, DerefMut};

use canonical::{Canon, CanonError, Id};

use crate::{Annotation, Compound};

#[cfg(feature = "persistance")]
use crate::Persisted;

#[derive(Debug, Clone)]
enum LinkInner<C, A> {
    Placeholder,
    C(Rc<C>),
    Ca(Rc<C>, A),
    #[allow(unused)] // TODO
    Ia(Id, A),
    #[allow(unused)] // TODO
    Ica(Id, Rc<C>, A),
}

impl<C, A> Default for LinkInner<C, A> {
    fn default() -> Self {
        LinkInner::Placeholder
    }
}

impl<C, A> Canon for Link<C, A>
where
    C: Compound<A> + Canon,
    A: Annotation<C::Leaf> + Canon,
{
    fn encode(&self, sink: &mut canonical::Sink) {
        self.id().encode(sink);
        self.annotation().encode(sink);
    }

    fn decode(source: &mut canonical::Source) -> Result<Self, CanonError> {
        let id = Id::decode(source)?;
        let a = A::decode(source)?;
        Ok(LinkInner::Ia(id, a).into())
    }

    fn encoded_len(&self) -> usize {
        self.id().encoded_len() + self.annotation().encoded_len()
    }
}

#[derive(Debug, Clone)]
/// A wrapper type that keeps the annotation of the Compound referenced cached
pub struct Link<C, A>(RefCell<LinkInner<C, A>>);

impl<C, A> From<LinkInner<C, A>> for Link<C, A> {
    fn from(inner: LinkInner<C, A>) -> Self {
        Link(RefCell::new(inner))
    }
}

impl<C, A> Link<C, A>
where
    C: Compound<A>,
    A: Annotation<C::Leaf>,
{
    /// Create a new annotated type
    pub fn new(compound: C) -> Self {
        LinkInner::C(Rc::new(compound)).into()
    }

    /// Returns a reference to to the annotation stored
    pub fn annotation(&self) -> LinkAnnotation<C, A> {
        let mut borrow = self.0.borrow_mut();
        let a = match *borrow {
            LinkInner::Ca(_, _)
            | LinkInner::Ica(_, _, _)
            | LinkInner::Ia(_, _) => return LinkAnnotation(borrow),
            LinkInner::C(ref c) => A::combine(c.annotations()),
            LinkInner::Placeholder => unreachable!(),
        };
        if let LinkInner::C(c) = mem::take(&mut *borrow) {
            *borrow = LinkInner::Ca(c, a)
        } else {
            unreachable!()
        }
        LinkAnnotation(borrow)
    }

    /// Gets a reference to the inner compound of the link'
    ///
    /// Can fail when trying to fetch data over i/o
    pub fn compound(&self) -> Result<LinkCompound<C, A>, CanonError> {
        let borrow: RefMut<LinkInner<C, A>> = self.0.borrow_mut();
        match *borrow {
            LinkInner::Placeholder => unreachable!(),
            LinkInner::C(_) | LinkInner::Ca(_, _) | LinkInner::Ica(_, _, _) => {
                return Ok(LinkCompound(borrow))
            }
            LinkInner::Ia(_, _) => todo!(),
        }
    }

    fn id(&self) -> Id {
        let borrow: RefMut<LinkInner<C, A>> = self.0.borrow_mut();
        match *borrow {
            LinkInner::Placeholder => unreachable!(),
            LinkInner::Ica(id, _, _) | LinkInner::Ia(id, _) => return id,
            _ => todo!(),
        }
    }

    /// Returns a Mutable reference to the underlying compound node
    ///
    /// Drops cached annotations and ids
    ///
    /// Can fail when trying to fetch data over i/o
    pub fn compound_mut(
        &mut self,
    ) -> Result<LinkCompoundMut<C, A>, CanonError> {
        let mut borrow: RefMut<LinkInner<C, A>> = self.0.borrow_mut();

        match mem::take(&mut *borrow) {
            LinkInner::Placeholder => unreachable!(),
            LinkInner::C(c) | LinkInner::Ca(c, _) | LinkInner::Ica(_, c, _) => {
                *borrow = LinkInner::C(c);
                return Ok(LinkCompoundMut(borrow));
            }

            LinkInner::Ia(_, _) => {
                todo!()
            }
        }
    }
}

/// A wrapped borrow of an inner link guaranteed to contain a computed
/// annotation
#[derive(Debug)]
pub struct LinkAnnotation<'a, C, A>(RefMut<'a, LinkInner<C, A>>);

/// A wrapped borrow of an inner node guaranteed to contain a compound node
#[derive(Debug)]
pub struct LinkCompound<'a, C, A>(RefMut<'a, LinkInner<C, A>>);

/// A wrapped mutable borrow of an inner node guaranteed to contain a compound
/// node
#[derive(Debug)]
pub struct LinkCompoundMut<'a, C, A>(RefMut<'a, LinkInner<C, A>>);

impl<'a, C, A> Deref for LinkAnnotation<'a, C, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        match *self.0 {
            LinkInner::Ica(_, _, ref a)
            | LinkInner::Ia(_, ref a)
            | LinkInner::Ca(_, ref a) => a,
            _ => unreachable!(),
        }
    }
}

impl<'a, C, A> Deref for LinkCompound<'a, C, A> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        match *self.0 {
            LinkInner::C(ref c)
            | LinkInner::Ca(ref c, _)
            | LinkInner::Ica(_, ref c, _) => c,
            _ => unreachable!(),
        }
    }
}

impl<'a, C, A> Deref for LinkCompoundMut<'a, C, A>
where
    C: Clone,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        match *self.0 {
            LinkInner::C(ref c) => c,
            _ => unreachable!(),
        }
    }
}

impl<'a, C, A> DerefMut for LinkCompoundMut<'a, C, A>
where
    C: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match *self.0 {
            LinkInner::C(ref mut c) => Rc::make_mut(c),
            _ => unreachable!(),
        }
    }
}
