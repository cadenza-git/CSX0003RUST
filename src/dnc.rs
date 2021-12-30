use std::fmt::Debug;

/// Merge subroutine
/// Join to slices while in the right Order
fn merge<T>(left: &[T], right: &[T]) -> (u32, Vec<T>)
    where T: Copy + Clone + Ord {

    use std::iter::Peekable;
    use std::cmp::Ordering;

    struct MergeIterator<I: Iterator> {
        left: Peekable<I>,
        right: Peekable<I>,
    }
    impl<I: Iterator> MergeIterator<I> {
        fn new(left: I, right: I) -> Self {
            MergeIterator {
                left: left.peekable(),
                right: right.peekable(),
            }
        }
    }
    impl<I> Iterator for MergeIterator<I>
        where I: Iterator, I::Item: Ord, {
        type Item = (u32, I::Item);

        fn next(&mut self) -> Option<Self::Item> {
            match
            match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => { Some(l.cmp(r)) },
                (Some(_), None) => Some(Ordering::Less),
                (None, Some(_)) => Some(Ordering::Greater),
                (None, None) => None,
            }
            {
                Some(Ordering::Equal) => Some((0, self.left.next().unwrap())),
                Some(Ordering::Less) => Some((0, self.left.next().unwrap())),
                Some(Ordering::Greater) => Some((1, self.right.next().unwrap())),
                None => None,
            }
        }
    }

    let (inv, miter): (Vec<u32>, Vec<T>) = MergeIterator::new(left.iter(),right.iter())
        .unzip();
    println!("Inv: {:?}", inv);
    (inv.into_iter().sum(), miter)
}

/// Sort function based on the merge sort algorithm
pub fn merge_sort<T>(v: &[T]) -> (u32, Vec<T>)
    where T: Copy + Clone + Ord + Debug {

    let len = v.len();
    let mut inversions: u32 = 0;

    println!("Input: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        0..=1 => (0, v.to_vec()),
        // sort the binary slice and exit
        // use a local variable to eliminate the need for &mut as input
        // and given we output a new vector
        2 => {
            let mut out = Vec::from(v);
            if out[0] > out[1] {
                out.swap(0, 1);
                inversions += 1;
            }
            (inversions, out)
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left,right) = v.split_at(len >> 1);
            let (linv, left) = merge_sort(left);
            let (rinv, right) = merge_sort(right);

            // return a vector of the merged but ordered slices
            let (minv, out) = merge(&left, &right);
            println!("Merged: {:?} <> {:?} => {}:{:?}", left, right, linv+rinv+minv, out);
            (linv + rinv + minv, out)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sort() {
        let v = vec![1,3,5,2,4];
        assert_eq!(merge_sort(&v), (3, vec![1,2,3,4,5]));
    }
    #[test]
    fn test_merge() {
        let s1 = &[2, 5, 7, 9];
        let s2 = &[1, 3, 6, 8];
        assert_eq!(merge(s1, s2).1, vec![1, 2, 3, 5, 6, 7, 8, 9]);
    }
}
