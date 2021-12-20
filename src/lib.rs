

/// Divide and Conquere algorithms
pub mod divnconq {

    /// Sort function based on the merge sort algorithm
    pub fn merge_sort(v: &[i32]) -> Vec<i32> {

        let len = v.len();
        println!("Input: ({}){:?} =>", len, v);
        match len {
            // unity slice, just return it
            0..=1 => v.to_vec(),
            // sort the binary slice and exit
            // use a local variable to eliminate the need for &mut as input
            // and given we output a new vector
            2 => {
                let mut out = Vec::from(v);
                if out[0] > out[1] {
                    out.swap(0, 1);
                }
                out
            },
            // if slice length longer than 2 then split recursively
            _ => {
                let (left,right) = v.split_at(len >> 1);
                let left = merge_sort(left);
                let right = merge_sort(right);

                // return a vector of the merged but ordered slices
                merge(&left, &right)
            }
        }
    }

    use std::iter::Peekable;
    use std::cmp::Ordering;

    struct MergeIterator<I: Iterator>
    {
        left: Peekable<I>,
        right: Peekable<I>,
    }


    impl<I: Iterator> MergeIterator<I>
    {
        fn new(left: I, right: I) -> Self {
            MergeIterator {
                left: left.peekable(),
                right: right.peekable(),
            }
        }
    }

    impl<I> Iterator for MergeIterator<I>
        where I: Iterator, I::Item: Ord,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            let which = match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => { Some(l.cmp(r)) },
                (Some(_), None) => Some(Ordering::Less),
                (None, Some(_)) => Some(Ordering::Greater),
                (None, None) => None,
            };

            match which {
                Some(Ordering::Equal) => self.left.next(),
                Some(Ordering::Less) => self.left.next(),
                Some(Ordering::Greater) => self.right.next(),
                None => None,
            }
        }
    }


    /// Merge subroutine
    /// Join to slices while in the right Order
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {

        let (l_len,r_len) = (left.len() - 1, right.len() - 1);

        let output = MergeIterator::new(left.iter(),right.iter()).cloned().collect();

        print!("merge: {},{:?} <> {},{:?},", r_len, right, l_len, left);
        println!("=> {:?},", output);
        output
    }

}