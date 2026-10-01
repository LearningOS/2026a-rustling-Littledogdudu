/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
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

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // Box::into_raw 所有权管理转为裸指针，如果没有内存可供分配就会直接 OOM panic，所以它一定不为空值
        // NonNull::new_unchecked 告诉编译器一定不为空值，编译器就不会为这个指针分配额外的字节判断是否为空值（用于Option<T>的None和Some判断）
        // node_ptr裸指针的占用空间就和C指针一致
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            // 空列表
            None => self.start = node_ptr,
            // 非空列表
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    /**
     * 此方法为合并a和b，直接剥夺了两个列表的所有权，在合并返回合并列表后两个列表本身从所有权和数据上均不可用
     */
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
    where
        T: PartialOrd
	{
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;
        let mut list_c = LinkedList::new();

        while let (Some(a), Some(b)) = (a_ptr, b_ptr) {
            let a_node: &mut Node<T> = unsafe { &mut (*a.as_ptr()) };
            let b_node: &mut Node<T> = unsafe { &mut (*b.as_ptr()) };
            // if a_node.val > b_node.val {
            //     if list_c.end.is_none() {
            //         list_c.start = b_ptr;
            //     } else {
            //         let end_node: &mut Node<T> = unsafe { &mut (*list_c.end.unwrap().as_ptr()) };
            //         end_node.next = b_ptr;
            //     }
            //     list_c.end = b_ptr;
            //     b_ptr = b_node.next;
            // } else {
            //     if list_c.end.is_none() {
            //         list_c.start = a_ptr;
            //     } else {
            //         let end_node: &mut Node<T> = unsafe { &mut (*list_c.end.unwrap().as_ptr()) };
            //         end_node.next = a_ptr;
            //     }
            //     list_c.end = a_ptr;
            //     a_ptr = a_node.next;
            // }
            let choosen = if a_node.val > b_node.val {
                b_ptr = b_node.next;
                b
            } else {
                a_ptr = a_node.next;
                a
            };
            match list_c.end {
                Some(end) => unsafe { (*end.as_ptr()).next = Some(choosen) },
                None => {
                    list_c.start = Some(choosen);
                }
            }
            list_c.end = Some(choosen);
        }

        // if a_ptr.is_none() {
        //     while let Some(b) = b_ptr {
        //         let b_node: &mut Node<T> = unsafe { &mut (*b.as_ptr()) };
        //         if list_c.end.is_none() {
        //             list_c.start = b_ptr;
        //         } else {
        //             let end_node: &mut Node<T> = unsafe { &mut (*list_c.end.unwrap().as_ptr()) };
        //             end_node.next = b_ptr;
        //         }
        //         list_c.end = b_ptr;
        //         b_ptr = b_node.next;
        //     }
        // } else {
        //     while let Some(a) = a_ptr {
        //         let a_node: &mut Node<T> = unsafe { &mut (*a.as_ptr()) };
        //         if list_c.end.is_none() {
        //             list_c.start = a_ptr;
        //         } else {
        //             let end_node: &mut Node<T> = unsafe { &mut (*list_c.end.unwrap().as_ptr()) };
        //             end_node.next = a_ptr;
        //         }
        //         list_c.end = a_ptr;
        //         a_ptr = a_node.next;
        //     }
        // }

        match (a_ptr, b_ptr) {
            (Some(a), None) => {
                if list_c.end.is_none() {
                    list_c.start = a_ptr;
                } else {
                    let end_node: &mut Node<T> = unsafe { &mut (*list_c.end.unwrap().as_ptr()) };
                    end_node.next = a_ptr;
                }
                list_c.end = list_a.end;
            },
            (None, Some(b)) => {
                if list_c.end.is_none() {
                    list_c.start = b_ptr;
                } else {
                    let end_node: &mut Node<T> = unsafe { &mut (*list_c.end.unwrap().as_ptr()) };
                    end_node.next = b_ptr;
                }
                list_c.end = b_ptr;
            },
            _ => ()
        }

        list_c.length = list_a.length + list_b.length;

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

    /**
     * 测试LinkedLisdt的add方法
     */
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

    /**
     * 测试merge，merge返回临时变量所有权
     */
    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}
