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
    if head.is_none() {
        return head;
    }
    let mut v_head = Box::new(ListNode::new(0));
    v_head.next = head;
    let mut cur = v_head.as_mut();
    let i = 1;
    while let Some(mut node) = cur.next.take() {
        // v_head = cur -> a(node) -> b(n_node)-> c -> None, 如果b存在，就交换
        if let Some(mut n_node) = node.next.take() {
            // v_head = cur -> a(node) -> c -> None
            node.next = n_node.next;
            // v_head = cur -> a(node) -> c -> None, b(n_node) -> a(node)
            n_node.next = Some(node);
            // v_head = cur -> b(n_node) -> a(node) -> c -> None
            cur.next = Some(n_node);
            // v_head -> b(n_node) -> a(node) = cur -> c -> None
            cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
        } else {
            // v_head -> b(n_node) -> a(node) = cur -> c -> None
            // 否则结束 cur => c -> None
            // v_head -> b -> a = cur -> c(node) -> None(n_node)
            cur.next = Some(node);
            // v_head -> b -> a -> cur = c(node) ->  None(n_node)
            cur = cur.next.as_mut().unwrap();
        }
    }
    v_head.next
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
