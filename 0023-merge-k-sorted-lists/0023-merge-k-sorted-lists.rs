use std::collections::BinaryHeap;
use std::cmp::Ordering;

impl PartialOrd<ListNode> for ListNode
{
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering>
    {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode
{
    fn cmp(&self, other: &ListNode) -> Ordering
    {
        other.val.cmp(&self.val)
    }
}

impl Solution
{
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>
    {
        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());

        // Initial nodes on heap
        for list in lists {
            if let Some(head) = list {
                heap.push(head);
            }
        }

        // Dummy and current, keep track of head and current node in result
        let mut dummy   = Box::new(ListNode::new(0));
        let mut curr    = &mut dummy;

        // Iterate through heap until empty
        while let Some(node) = heap.pop() {

            let mut new_node = Box::new(ListNode::new(node.val));
            curr.next = Some(new_node);
            curr = curr.next.as_mut().unwrap();

            // Push next element in list to heap
            if let Some(next) = node.next {
                heap.push(next);
            }
        }

        dummy.next
    }
}