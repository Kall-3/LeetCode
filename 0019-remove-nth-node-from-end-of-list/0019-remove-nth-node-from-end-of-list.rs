// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {

        // Find size of list
        let mut current = &mut head;
        let mut size = 0;

        while let Some(node) = current {
            current = &mut node.next;
            size += 1;
        }
        
        // Exit if size smaller than target
        if size < n {
            return head;
        }
        // If the list is too short
        if size < 2 {
            return None;
        } 
        // If we need to remove the head of the list
        if size == n {
            return head.unwrap().next;
        }
        
        // Find predecessor to n-th element
        let n = size - n;
        let mut current = &mut head;

        for _ in 0..n-1 {
            // SAFETY: Know size so never out of bounds
            current = &mut current.as_mut().unwrap().next;
        }

        // Move next pointer to skip over n-th element
        if let Some(prev) = current {
            let node = prev.next.take();
            prev.next = node.unwrap().next;
        }

        head
    }
}