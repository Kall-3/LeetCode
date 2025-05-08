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

use std::collections::BinaryHeap;
use std::cmp::{Reverse, Ordering};

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &ListNode) -> Ordering {
        other.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::new();

        let mut head = Some(Box::new(ListNode::new(0)));
        for mut list in lists {
            head = list.take();

            while let Some(mut node) = head {
                let next = node.next.take();
                heap.push(node);
                head = next;
            }
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur = dummy.as_mut();

        while let Some(node) = heap.pop() {
            if let Some(current) = cur {
                current.next = Some(node);
                cur = current.next.as_mut();
            }
        }

        dummy.unwrap().next
    }
}