// 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。
//
//
// 示例 1：
//
//
// 输入：head = [1,2,6,3,4,5,6], val = 6
// 输出：[1,2,3,4,5]
// 示例 2：
//
// 输入：head = [], val = 1
// 输出：[]
// 示例 3：
//
// 输入：head = [7,7,7,7], val = 7
// 输出：[]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
// a -> b -> c b
// 0 -> a -> b -> c -> d
// a -> c -> d
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    dummy_head.next = head;
    let mut cur = dummy_head.as_mut();
    while let Some(next_node) = cur.next.take() {
        if next_node.val == val {
            cur.next = next_node.next;
        } else {
            cur.next = Some(next_node);
            cur = cur.next.as_mut().unwrap();
        }
    }
    dummy_head.next
}