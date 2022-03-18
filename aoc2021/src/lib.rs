use std::iter::Enumerate;

///
/// trim_empty for lines
///
pub struct TrimEmptyIter<I> {
    pub iter: I,
}

impl<'a, I, T> Iterator for TrimEmptyIter<I>
    where
        I: Iterator<Item=&'a T>,
        T: AsRef<str> + 'a + ?Sized
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(s) => {
                    let s = s.as_ref().trim();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
        }
    }
}

// pub trait TrimEmpty {
//     type Iter;
//
//     fn trim_empty(self) -> Self::Iter;
// }
//
// impl<'a, I, T> TrimEmpty for I
//     where
//         I: Iterator<Item=&'a T>,
//         T: AsRef<str> + 'a + ?Sized,
// {
//     type Iter = TrimEmptyIter<I>;
//
//     fn trim_empty(self) -> TrimEmptyIter<I> {
//         TrimEmptyIter { iter: self }
//     }
// }

pub trait TrimEmpty: Iterator + Sized {
    fn trim_empty<'a, T>(self) -> TrimEmptyIter<Self>
        where
            Self: Iterator<Item=&'a T>,
            T: AsRef<str> + 'a + ?Sized,
    {
        TrimEmptyIter { iter: self }
    }
}

impl<'a, T> TrimEmpty for T where T: Iterator {}

///
/// enumerate_2d for 2d iterators
///

pub struct Iter2DEnumerate<I, J> {
    ys: Enumerate<I>,
    xs: Option<(usize, Enumerate<J>)>,
}

impl<RowIter, ColIter> Iterator for Iter2DEnumerate<RowIter, ColIter>
    where
        RowIter: Iterator,
        RowIter::Item: IntoIterator<Item=ColIter::Item, IntoIter=ColIter>,
        ColIter: Iterator,
{
    type Item = (usize, usize, ColIter::Item);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next) = self
                .xs
                .as_mut()
                .and_then(|(y, xs)| xs.next().map(|(x, item)| (*y, x, item)))
            {
                return Some(next);
            }

            if let Some((y, xs)) = self.ys.next() {
                self.xs = Some((y, xs.into_iter().enumerate()));
            } else {
                return None;
            }
        }
    }
}

pub trait Enumerate2D: Iterator + Sized {
    fn enumerate_2d<ColIter>(self) -> Iter2DEnumerate<Self, ColIter>
        where
            ColIter: Iterator,
            Self::Item: IntoIterator<Item=ColIter::Item, IntoIter=ColIter>,
    {
        Iter2DEnumerate {
            ys: self.enumerate(),
            xs: None,
        }
    }
}

impl<T> Enumerate2D for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_trim_empty() {
        let in1 = vec!["a", " b ", "  ", "c  "];
        let in2 = vec!["a".to_string(), " b ".to_string(), "  ".to_string(), "c  ".to_string()];
        let in3 = "a\n b \n\n  c  \n";
        let in4 = "   a   b   c   ";
        let out = vec!["a", "b", "c"];

        assert_eq!(out, in1.iter().trim_empty().collect_vec());
        assert_eq!(out, in2.iter().trim_empty().collect_vec());
        assert_eq!(out, in3.lines().trim_empty().collect_vec());
        assert_eq!(out, in4.split(' ').trim_empty().collect_vec());
    }

    #[test]
    fn test_enumerate_2d() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let out = vec![(0, 0, &1), (0, 1, &2), (1, 0, &3), (1, 1, &4)];

        assert_eq!(out, v.iter().enumerate_2d().collect_vec());
    }
}
