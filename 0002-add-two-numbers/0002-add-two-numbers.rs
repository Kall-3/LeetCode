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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut num = 0;
        let mut carry = 0;
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut head = l3.as_mut();

        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

        while l1.is_some() || l2.is_some() {
            num = 0;

            if let Some(node) = l1 {
                num     += node.val;
                l1      = node.next.as_ref();
            }

            if let Some(node) = l2 {
                num     += node.val;
                l2      = node.next.as_ref();
            }

            num     += carry;
            carry   = num / 10;
            num     = num % 10;

            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(num)));
            head = head.unwrap().next.as_mut();
        }

        if carry != 0 {
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        
        
        l3.unwrap().next
    }
}