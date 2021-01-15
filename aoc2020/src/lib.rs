///
/// trim_empty for lines
///

pub struct TrimEmptyIter<I> {
    iter: I
}

impl<'a, I> Iterator for TrimEmptyIter<I>
    where I: Iterator<Item=&'a str>
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(s) => {
                    let s = s.trim();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
        }
    }
}

pub trait TrimEmpty {
    type Iter;

    fn trim_empty(self) -> Self::Iter;
}

impl<'a, I> TrimEmpty for I
    where I: Iterator<Item=&'a str>
{
    type Iter = TrimEmptyIter<I>;

    fn trim_empty(self) -> TrimEmptyIter<I> {
        TrimEmptyIter { iter: self }
    }
}

//
// pub struct Iter2DEnumerate<I, J> {
//     pub ys: I,
//     pub y_cur: Option<usize>,
//     pub xs: Option<J>,
// }
//
// impl<'a, I, J> Iterator for Iter2DEnumerate<I, J>
//     where I: Iterator<Item=(usize, &'a Vec<i32>)>,
//           J: Iterator<Item=(usize, &'a i32)>,
// {
//     type Item = (usize, usize, &'a i32);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         assert!((self.y_cur.is_none() && self.xs.is_none())
//             || (self.y_cur.is_some() && self.xs.is_some()));
//         loop {
//             if self.y_cur.is_none() {
//                 match self.ys.next() {
//                     None => return None,
//                     Some((y, xs)) => {
//                         self.y_cur = Some(y);
//                         self.xs = Some(xs.iter().enumerate());
//                     }
//                 }
//             }
//             match self.xs.unwrap().next() {
//                 None => self.y_cur = None,
//                 Some((x, elem)) => return Some((self.y_cur.unwrap(), x, elem))
//             }
//         }
//     }
// }
