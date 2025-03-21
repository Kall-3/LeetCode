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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }

        // Find mutable reference to middle
        let mut len = 0;
        let mut current = head.as_ref();

        while let Some(node) = current {
            current = node.next.as_ref();
            len += 1;
        }

        let mut current = head.as_mut();

        for i in 0..(len + 1)/2 - 1 {
            current = if let Some(node) = current {
                node.next.as_mut()
            } else {
                return;
            };
        }

        // Reverse second half
        let mut prev = None;
        let mut current = current.unwrap().next.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        // Merge halfs
        let mut second_half = prev.take();

        let mut current = head.as_mut();

        while let (Some(node), Some(mut second)) = (current, second_half) {
            let next = node.next.take();
            let next_second = second.next.take();

            second.next = next;
            node.next = Some(second);

            current = node.next.as_mut().unwrap().next.as_mut();

            second_half = next_second;
        }
    }
}