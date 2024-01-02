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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let val1 = list1.as_mut().unwrap().val;
            let val2 = list2.as_mut().unwrap().val;
            
            if val1 <= val2 {
                current.next = list1.take();
                list1 = current.next.as_mut().unwrap().next.take();
            } else {
                current.next = list2.take();
                list2 = current.next.as_mut().unwrap().next.take();
            }

            current = current.next.as_mut().unwrap();
        }

        current.next = list1.or(list2);

        
        dummy.next
    }
}