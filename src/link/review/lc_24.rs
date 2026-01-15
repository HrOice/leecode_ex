// 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。
//
//
//
// 示例 1：
//
//
// 输入：head = [1,2,3,4]
// 输出：[2,1,4,3]
// 1,2,3,4
// 2,1,3,4
// 2,1,4,3

// 示例 2：
//
// 输入：head = []
// 输出：[]
// 示例 3：
//
// 输入：head = [1]
// 输出：[1]

// Definition for singly-linked list.

use crate::link::{build_list, print_list, ListNode};

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode {val: 0, next: head});
    let mut cur = &mut dummy_head;
    while let Some(mut node) = cur.next.take() {
        if let Some(mut n_node) = node.next.take() {
            node.next = n_node.next;
            n_node.next = Some(node);
            cur.next = Some(n_node);
            cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
        } else {
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
    dummy_head.next
}

#[test]
fn test() {
    let head = build_list(&[1, 2, 3, 4, 5]);
    print!("original: ");
    print_list(head.clone());

    let reversed = swap_pairs(head);
    print!("reversed: ");
    print_list(reversed);
}
