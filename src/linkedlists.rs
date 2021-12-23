
/// I could have done it differently
/// but I wanted to see how far I could go with this simple enum structure
#[derive(Debug, PartialEq)]
pub enum List<T>
    where T: Copy + Clone + PartialEq {
    Empty,
    NotEmpty(T, Box<List<T>>),
}

/// List related methods
impl<T> List<T>
    where T: Copy + Clone + PartialEq {

    /// Construct an empty list
    pub fn new() -> List<T> {
        List::Empty
    }
    /// Push to the end of the list. inefficiently for now
    pub fn push(&mut self, item: T) {
        match self {
            List::Empty => {
                *self = List::NotEmpty(
                    item,
                    Box::new(List::Empty)
                );
            }
            List::NotEmpty(_, next) => {
                next.push(item);
            }
        }
    }
    /// Pop from the end of the list, inefficiently for now
    pub fn pop(&mut self) -> Option<T> {
        match self {
            List::Empty => None,
            List::NotEmpty(val, next) => {
                if List::Empty == **next {
                    let item = *val;
                    *self = List::Empty;
                    Some(item)
                }
                else {
                    next.pop()
                }
            }
        }
    }
    pub fn iter(&self) -> ListIterByRef<T> {
        match self {
            List::Empty =>
                ListIterByRef {
                    cursor: &List::Empty,
                },
            List::NotEmpty(_, _) =>
                ListIterByRef {
                    cursor: self
                },
        }
    }
}

/// List provides a "non-consuming" iterator... against the norm
/// '''
/// for i in &list
/// '''
impl<T> IntoIterator for List<T>
    where T: Copy + Clone + PartialEq {

    type Item = T;
    type IntoIter = ListIter<T>;

    /// Here we are taking ownership of self hence destroying the node
    /// when we go out of scope.
    /// Self contents are MOVED onto ListIter structure
    /// hence ListIter becomes the list's head
    fn into_iter(self) -> Self::IntoIter {
        // self taken by value, hence consumed
        match self {
            List::Empty => ListIter { cursor: List::Empty },
            // as a result val & next are consumed in this scope
            List::NotEmpty(val, next) => {
                ListIter {
                    // val & next are moved into cursor
                    cursor: List::NotEmpty(
                        val,
                        next,
                    )
                }
            }
        }
    }
}

/// A List can be constructed from other collections
/// '''
/// let mut l : List<i32> = v.into_iter().collect();
/// '''
impl<T> FromIterator<T> for List<T>
    where T: Copy + Clone + PartialEq {

    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut list = List::Empty;
        for item in iter {
            list.push(item);
        }
        list
    }
}

/// List by reference iterator
pub struct ListIterByRef<'a, T>
    where T: Copy + Clone + PartialEq {
    cursor: &'a List<T>,
}

impl<'a, T> Iterator for ListIterByRef<'a, T>
    where T: Copy + Clone + PartialEq {

    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            List::Empty => None,
            List::NotEmpty(value, next) => {
                self.cursor = next;
                Some(value)
            }
        }
    }
}

/// A List Iterator
/// Sets up a cursor that references current node
pub struct ListIter<T>
    where T: Copy + Clone + PartialEq {
    cursor: List<T>,
}

impl<T> Iterator for ListIter<T>
    where T: Copy + Clone + PartialEq {

    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            List::Empty => None,
            List::NotEmpty(value, ref mut next) => {
                // "next" deconstructed from "self",
                // however we need *next copied onto self BUT Box cannot be copied
                // given "self" is consumed and destroyed when exiting this scope
                // we need to construct a new "tmp" List Node
                // copy *next contents into this tmp Node
                // then make self take onwership of tmp Node
                // (that is, self points to new mem location) !
                let mut tmp = Box::new( List::Empty );
                std::mem::swap( next, &mut tmp);
                self.cursor = *tmp;
                Some(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list,
                   List::NotEmpty(1,Box::new(
                       List::NotEmpty(2, Box::new(
                           List::NotEmpty(3, Box::new(
                               List::Empty
                           ))
                       ))
                   ))
        )
    }
    #[test]
    fn test_pop() {
        let mut l = List::new();
        l.push(1);
        l.push(2);

        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
        assert_eq!(l.pop(), None);
    }
    #[test]
    fn test_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);

        let mut iter = l.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        let m: List<i32> = List::new();
        assert_eq!(m.iter().cursor, &List::Empty);
    }
    #[test]
    fn test_into_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);

        let mut iter = l.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);

        let l:List<i32> = List::new();
        assert_eq!(l.into_iter().cursor, List::Empty);
    }
    #[test]
    fn test_from_iter() {
        let v = vec![1,2,3];

        let mut l : List<i32> = v.into_iter().collect();

        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }
}