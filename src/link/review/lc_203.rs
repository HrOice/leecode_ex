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

use crate::link::{build_list, print_list, ListNode};

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode {val: 0, next: head});
    let mut cur = &mut dummy_head;
    // cur 需要指向目标的前置
    while let Some(node) = cur.next.take() { //
        if node.val == val {
            cur.next = node.next;
        } else {
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
    dummy_head.next
}

#[test]
fn test() {
    let head = build_list(&[1,2,6,6,6]);
    let res = remove_elements(head, 6);
    print_list(res);
}