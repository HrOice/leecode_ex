use crate::link::{build_list, print_list, ListNode};

// 题意：反转一个单链表。
//
// 示例: 输入: 1->2->3->4->5->NULL 输出: 5->4->3->2->1->NULL

// Definition for singly-linked list.

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // 需要更改next方向，需要一个指向刚刚变更过的节点
    let mut prev = None;
    let mut cur = head;
    while let Some(mut node) = cur.take() {//
        cur = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

#[test]
fn test() {
    let head = build_list(&[1, 2, 3, 4, 5]);
    print!("original: ");
    print_list(head.clone());

    let reversed = reverse_list(head);
    print!("reversed: ");
    print_list(reversed);
}
