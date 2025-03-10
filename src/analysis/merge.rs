use std::cmp::Ordering;
use std::iter::Peekable;

pub struct MergeOr<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
{
    left: Peekable<L>,
    right: Peekable<R>,
}

impl<L, R> MergeOr<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
{
    pub(crate) fn new(left: L, right: R) -> Self {
        MergeOr {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}

impl<L, R> Iterator for MergeOr<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
    L::Item: Ord,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<L::Item> {
        let which = match (self.left.peek(), self.right.peek()) {
            (Some(l), Some(r)) => Some(l.cmp(r)),
            (Some(_), None) => Some(Ordering::Less),
            (None, Some(_)) => Some(Ordering::Greater),
            (None, None) => None,
        };

        match which {
            Some(Ordering::Less) => self.left.next(),
            Some(Ordering::Equal) => {
                self.left.next();
                self.right.next()
            }
            Some(Ordering::Greater) => self.right.next(),
            None => None,
        }
    }
}

pub struct MergeAnd<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
{
    left: Peekable<L>,
    right: Peekable<R>,
}

impl<L, R> MergeAnd<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
{
    pub(crate) fn new(left: L, right: R) -> Self {
        MergeAnd {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}

impl<L, R> Iterator for MergeAnd<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
    L::Item: Ord,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<L::Item> {
        loop {
            match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => match l.cmp(r) {
                    Ordering::Less => {
                        self.left.next();
                    }
                    Ordering::Equal => {
                        self.right.next();
                        return self.left.next();
                    }
                    Ordering::Greater => {
                        self.right.next();
                    }
                },
                (Some(_), None) => {
                    self.left.next();
                }
                (None, Some(_)) => {
                    self.right.next();
                }
                (None, None) => return None,
            }
        }
    }
}
