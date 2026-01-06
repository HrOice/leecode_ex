// 题意： 给定一个链表，返回链表开始入环的第一个节点。 如果链表无环，则返回 null。
//
// 为了表示给定链表中的环，使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。 如果 pos 是 -1，则在该链表中没有环。
//
// 说明：不允许修改给定的链表。

// 输入：head = [3,2,0,-4], pos = 1
// 输出：返回索引为 1 的链表节点
// 解释：链表中有一个环，其尾部连接到第二个节点。

// 输入：head = [1,2], pos = 0
// 输出：返回索引为 0 的链表节点
// 解释：链表中有一个环，其尾部连接到第一个节点。

use std::cell::RefCell;
use std::rc::Rc;
use crate::link::leecode_interview_02_07::{print_list, ListNode};
pub(crate) fn build_list(vals: &[i32], n: usize) -> crate::link::leecode_interview_02_07::Link {
    let mut head: crate::link::leecode_interview_02_07::Link = None;
    let mut tail: crate::link::leecode_interview_02_07::Link = None;
    let mut n = n;
    let mut cur = None;

    for &v in vals {
        let node = Rc::new(RefCell::new(ListNode {
            val: v,
            next: None,
        }));

        if head.is_none() {
            head = Some(node.clone());
        } else {
            tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }

        tail = Some(node.clone());
        if n == 0 && cur.is_none() {
            cur = Some(node);
        } else {
            n -= 1;
        }
    }
    tail.unwrap().borrow_mut().next = cur;

    head
}
#[test]
fn test() {
    // list1
    let list1_head = build_list(&[3,2,0,-4], 1); // 1,2,6,11,3,4,5

    // let reversed = crate::link::leecode_interview_02_07::find_intersect(list1_head, list2_head);
    print!("reversed: ");
    // print_list(reversed);
}
