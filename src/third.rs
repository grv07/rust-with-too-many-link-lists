use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

pub type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        let new_node = Rc::new(Node {
            elem,
            next: self.head.clone(),
        });

        Self {
            head: Some(new_node),
        }
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let resp = self.next.map(|node| &node.elem);
        self.next = self.next.and_then(|node| node.next.as_deref());

        resp
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn create() {
        let list = List::new();
        let list1 = list.prepend(1);
        let list2 = list1.prepend(2);

        assert_eq!(list.head(), None);
        assert_eq!(list1.head(), Some(&1));
        assert_eq!(list2.head(), Some(&2));
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_ne!(iter.next(), Some(&1));
    }
}
