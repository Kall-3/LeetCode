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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut rest = 0;
        let mut num = 0;

        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        
        while l1.is_some() || l2.is_some() {
            num = 0;

            if let Some(node) = l1 {
                num += node.val;
                l1 = node.next.as_ref();
            }

            if let Some(node) = l2 {
                num += node.val;
                l2 = node.next.as_ref();
            }

            num += rest;
            rest = num / 10;

            tail.next = Some(Box::new(ListNode::new(num % 10)));

            tail = tail.next.as_mut().unwrap();
        }


        if rest > 0 {
            tail.next = Some(Box::new(ListNode::new(rest)));
        }

        head.next
    }
}