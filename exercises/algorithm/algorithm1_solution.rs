/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list

    Solution: iterative merge that splices the existing nodes (O(n + m) time, O(1) extra space).
    A raw tail pointer keeps `add` at O(1) so the merge itself is a straight walk of both lists.
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    /// Raw pointer to the last node; keeps `add` O(1).
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    /// Append to the tail. `Box` moves do not change the heap address, so the
    /// `NonNull` stored in `end` stays valid after the box is moved into the list.
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    /// Iterative lookup instead of the original recursive one (no stack growth).
    pub fn get(&self, index: i32) -> Option<&T> {
        let mut current = self.start;
        let mut i = index;
        while i > 0 {
            current = current.map(|node| unsafe { (*node.as_ptr()).next })?;
            i -= 1;
        }
        current.map(|node| unsafe { &(*node.as_ptr()).val })
    }

    /// Merge two sorted lists by relinking their nodes. Neither input list may be
    /// used afterwards: its nodes now belong to the returned list.
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd,
    {
        let length = list_a.length + list_b.length;

        // `start` is moved out, so the inputs never drop their nodes (no double free).
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;

        let mut list_c = LinkedList::new();

        loop {
            let take_a = match (&a_ptr, &b_ptr) {
                (Some(a), Some(b)) => {
                    let (a, b) = unsafe { (&*a.as_ptr(), &*b.as_ptr()) };
                    a.val <= b.val // `<=` keeps the merge stable
                }
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (None, None) => break,
            };

            // Detach the chosen head. `NonNull` is `Copy`, so the same value can
            // be used both as the link and as the new tail pointer.
            let head = if take_a {
                let head = a_ptr.take().unwrap();
                a_ptr = unsafe { (*head.as_ptr()).next };
                head
            } else {
                let head = b_ptr.take().unwrap();
                b_ptr = unsafe { (*head.as_ptr()).next };
                head
            };

            match list_c.end {
                Some(end) => unsafe { (*end.as_ptr()).next = Some(head) },
                None => list_c.start = Some(head),
            }
            list_c.end = Some(head);
        }

        list_c.length = length;
        list_c
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
