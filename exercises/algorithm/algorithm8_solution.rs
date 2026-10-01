/*
    queue
    This question requires you to use queues to implement the functionality of the stac

    Solution: keep every element in `q1`. To pop, move all but the last element
    into `q2`, dequeue the last one (the stack top), then swap the two queues.
    push is O(1), pop is O(n) — the usual two-queue trade-off.
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

#[allow(non_camel_case_types)]
pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // Rotate every element except the last one into the helper queue.
        while self.q1.size() > 1 {
            // `dequeue` ties its `&str` error to `&mut self`, so handle it here
            // instead of with `?` (which would keep q1 borrowed).
            let elem = match self.q1.dequeue() {
                Ok(elem) => elem,
                Err(_) => break,
            };
            self.q2.enqueue(elem);
        }

        // The last element of q1 is the top of the stack.
        let top = match self.q1.dequeue() {
            Ok(top) => top,
            Err(_) => return Err("Stack is empty"),
        };

        // q2 now holds all remaining elements; make it the primary queue again.
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(top)
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
