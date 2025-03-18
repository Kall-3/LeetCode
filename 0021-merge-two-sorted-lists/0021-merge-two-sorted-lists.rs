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
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;

        while let (Some(node1_ref), Some(node2_ref)) = (list1.as_ref(), list2.as_ref()) {
            if node1_ref.val < node2_ref.val {
                current.next = list1.take();
                current = current.next.as_mut().unwrap();
                list1 = current.next.take();
            } else {
                current.next = list2.take();
                current = current.next.as_mut().unwrap();
                list2 = current.next.take();
            }
        }

        current.next = list1.or(list2);

        head.next
    }
}