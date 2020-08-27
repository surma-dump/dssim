pub mod prelude {
    pub use super::*;
}

pub trait ParallelIterator: Iterator + Sized {
    fn with_max_len(self, _l: usize) -> Self {
        self
    }
    fn reduce_with<OP>(mut self, op: OP) -> Option<Self::Item>
    where
        OP: Fn(Self::Item, Self::Item) -> Self::Item + Sync,
    {
        if let Some(a) = self.next() {
            Some(self.fold(a, op))
        } else {
            None
        }
    }
}

pub trait IntoParallelIterator {
    type Iter: Iterator<Item = Self::Item>;
    type Item: Send;
    fn into_par_iter(self) -> Self::Iter;
}

pub trait IntoParallelRefIterator<'data> {
    type Iter: Iterator<Item = Self::Item>;
    type Item: Send + 'data;
    fn par_iter(&'data self) -> Self::Iter;
}

impl<I: IntoIterator> IntoParallelIterator for I
where
    I::Item: Send,
{
    type Iter = I::IntoIter;
    type Item = I::Item;

    fn into_par_iter(self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'data, I: 'data + ?Sized> IntoParallelRefIterator<'data> for I
where
    &'data I: IntoParallelIterator,
{
    type Iter = <&'data I as IntoParallelIterator>::Iter;
    type Item = <&'data I as IntoParallelIterator>::Item;

    fn par_iter(&'data self) -> Self::Iter {
        self.into_par_iter()
    }
}

impl<I: Iterator> ParallelIterator for I {}

pub fn join<A, B>(a: impl FnOnce() -> A, b: impl FnOnce() -> B) -> (A, B) {
    (a(), b())
}

pub trait ZipEq<A: IntoParallelIterator> {

    fn zip_eq<B: IntoParallelIterator>(self, other: B) -> std::iter::Zip<A::Item, B::Item> {
        // Totally ignoring the `_eq` part lol
        self.into_par_iter().zip(other.into_par_iter()) 
    }
}

impl<I: IntoParallelIterator> ZipEq<I> for I { }
