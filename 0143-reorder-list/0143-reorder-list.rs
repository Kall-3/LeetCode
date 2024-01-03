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

use std::hint::unreachable_unchecked;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Find how many nodes are in the list
        let mut count = 0;
        let mut list = &*head;

        while let Some(node) = list {
            list = &node.next;
            count += 1;
        }

        // If there are less than two nodes, then there is nothing to do
        if count <= 2 {
            return;
        }

        // Reach the middle of the list in order to split in to two lists
        let mut half = head.as_mut();
        for _ in 0..count / 2 {
            match half {
                None => unsafe { unreachable_unchecked() },
                Some(node) => {
                    half = node.next.as_mut();
                }
            }
        }

        // Reverse the second half
        let mut half = match half {
            None => unsafe { unreachable_unchecked() },
            Some(node) => node.next.take(),
        };

        let mut reversed = ListNode::new(0);
        while let Some(mut node) = half.take() {
            half = node.next.take();
            node.next = reversed.next.take();
            reversed.next = Some(node);
        }

        let mut tail = match head.as_mut() {
            None => unsafe { unreachable_unchecked() },
            Some(node) => &mut node.next,
        };

        while tail.is_some() && reversed.next.is_some() {
            let mut rev = reversed.next.take().unwrap();
            reversed.next = rev.next.take();

            rev.next = tail.take();
            *tail = Some(rev);
            tail = &mut tail.as_mut().unwrap().next;
            if let Some(node) = tail {
                tail = &mut node.next;
            }
        }
    }
}       