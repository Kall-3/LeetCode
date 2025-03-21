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
        let mut new_head = Some(Box::new(ListNode::new(0)));
        new_head.as_mut().unwrap().next = head.take();

        let mut current = new_head.as_ref().unwrap().next.as_ref();
        let mut c = 0;

        while let Some(node) = current {
            current = node.next.as_ref();
            c += 1;
        }

        if c < 2 {
            return None;
        }

        let mut current = new_head.as_mut();

        for _ in 0..(c - n) {
            current = current.unwrap().next.as_mut();
        }

        let next_node = current.as_mut().unwrap().next.take();
        current.as_mut().unwrap().next = next_node.and_then(|n| n.next);

        new_head.unwrap().next
    }
}