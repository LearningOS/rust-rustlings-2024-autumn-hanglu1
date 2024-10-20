/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node { val, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: usize,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> 
where
    T: std::cmp::PartialOrd + Clone, // Ensure these bounds are specified
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> LinkedList<T>
where
    T: std::cmp::PartialOrd + Clone,
{
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }

        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: usize) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => {
                if index == 0 {
                    Some(unsafe { &(*next_ptr.as_ptr()).val })
                } else {
                    self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1)
                }
            }
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: std::cmp::PartialOrd + Clone,  // Ensure bounds are here too
    {
        let mut merged_list = Self::new();
        let mut node_a = list_a.start;
        let mut node_b = list_b.start;

        while let (Some(a_ptr), Some(b_ptr)) = (node_a, node_b) {
            let a_val = unsafe { &*a_ptr.as_ptr() }.val.clone(); // Use clone
            let b_val = unsafe { &*b_ptr.as_ptr() }.val.clone(); // Use clone

            if a_val < b_val {
                merged_list.add(a_val);
                node_a = unsafe { &*a_ptr.as_ptr() }.next;
            } else {
                merged_list.add(b_val);
                node_b = unsafe { &*b_ptr.as_ptr() }.next;
            }
        }

        while let Some(node) = node_a {
            let val = unsafe { &*node.as_ptr() }.val.clone(); // Use clone
            merged_list.add(val);
            node_a = unsafe { &*node.as_ptr() }.next;
        }

        while let Some(node) = node_b {
            let val = unsafe { &*node.as_ptr() }.val.clone(); // Use clone
            merged_list.add(val);
            node_b = unsafe { &*node.as_ptr() }.next;
        }

        merged_list.length = list_a.length + list_b.length;
        merged_list
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

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &val) in target_vec.iter().enumerate() {
            assert_eq!(val, *list_c.get(i).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &val) in target_vec.iter().enumerate() {
            assert_eq!(val, *list_c.get(i).unwrap());
        }
    }
}
