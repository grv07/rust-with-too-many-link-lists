use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            next: None,
            prev: None,
        }))
    }
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_node| {
            match old_node.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            Rc::try_unwrap(old_node).ok().unwrap().into_inner().elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<'_, T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |r| &r.elem))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<'_, T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |r| &mut r.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<'_, T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |r| &r.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<'_, T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |r| &mut r.elem))
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));

        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek_front_test() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(*list.peek_front().unwrap(), 2);
    }

    #[test]
    fn push_back() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        // push_back
        list.push_back(3);

        assert_eq!(*list.peek_front().unwrap(), 2);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn pop_back() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(1));

        assert_eq!(list.pop_front(), None);
    }
}
