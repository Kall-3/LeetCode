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
use std::collections::VecDeque;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut deque: VecDeque<Box<ListNode>> = VecDeque::with_capacity(k as usize);

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur_h = dummy.as_mut();

        loop {
            let mut i = 0;

            while let Some(mut node) = cur {
                let next = node.next.take();
                deque.push_front(node);
                cur = next;

                i += 1;

                if i == k {
                    break;
                }
            }

            if cur == None && i != k {
                break;
            }

            while let Some(node) = deque.pop_front() {
                if let Some(current) = cur_h {
                    current.next = Some(node);
                    cur_h = current.next.as_mut();
                }
            }
        }

        while let Some(node) = deque.pop_back() {
            if let Some(mut current) = cur_h {
                current.next = Some(node);
                cur_h = current.next.as_mut();
            }
        }

        dummy.unwrap().next
    }
}