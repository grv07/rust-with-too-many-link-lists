use std::mem;

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;
// enum Link {
//     Empty,
//     More(Box<Node>),
// }

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut next_node = mem::replace(&mut self.head, None);

        while let Some(node) = &mut next_node {
            next_node = mem::replace(&mut node.next, None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);
    }
}
