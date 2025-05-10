pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head) = &self.head {
            return Some(&head.elem);
        }
        None
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if let Some(head) = &mut self.head {
            return Some(&mut head.elem);
        }

        None
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            // next: self.head.as_ref().map::<&Node<T>, _>(|node| &node),
            // next: self.head.as_ref().map(|node| &**node),
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next {
            let res = Some(&node.elem);
            self.next = node.next.as_deref();
            res
        } else {
            None
        }
    }
}

pub struct MutIter<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn mut_iter(&mut self) -> MutIter<T> {
        MutIter {
            next: self.head.as_mut().map(|node| node.as_mut()),
        }
    }
}

impl<'a, T> Iterator for MutIter<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(node) => {
                self.next = node.next.as_deref_mut();
                Some(&mut node.elem)
            }
            None => None,
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next_node = self.head.take();

        while let Some(node) = &mut next_node {
            next_node = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|v| {
            *v = 90;
        });

        assert_ne!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::<i32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn iter() {
        let mut list = List::<i32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mut_iter() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.mut_iter();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);

        let mut iter = list.mut_iter();
        assert_eq!(iter.next(), Some(&mut 3));
    }
}
