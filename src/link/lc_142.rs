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
pub(crate) fn build_list(vals: &[i32], n: i32) -> crate::link::leecode_interview_02_07::Link {
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
        } else if n > 0 {
            n -= 1;
        }
    }
    if n < 0 {
        return head;
    }
    tail.unwrap().borrow_mut().next = cur;

    head
}


// a -> b -> c -> d -> e -> f -> g -> d
// a ----x ------d --- y ---xxx ---- z -> d
// slow 1 step / fast 2 step
// 相遇在环内 xxx
// slow : x + y
// fast : x + y + n*(y + z)
// slow * 2 = fast
// 2 (x+y) = x + y + n(y + z)
// x + y = ny + nz
// x = (n-1)y + nz = (n - 1)(y+z) + z
// y + z 是完整一圈
// 在相遇点开始 slow 继续走，slow2 从起点开始走，再次相遇时就是环的起点
fn find_ring_start(head: crate::link::leecode_interview_02_07::Link) -> crate::link::leecode_interview_02_07::Link {
    let mut slow = head.clone();
    let mut fast = head.clone();
    loop {
        slow = slow.unwrap().borrow_mut().next.clone();
        fast = fast.unwrap().borrow_mut().next.clone();
        if fast.is_none() {
            return None;
        }
        fast = fast.unwrap().borrow_mut().next.clone();
        if fast.is_none() {
            return None;
        }
        if fast == slow {
            break;
        }
    }
    if head == slow {
        return head;
    }
    let mut slow2 = head.clone();
    loop {
        slow = slow.unwrap().borrow_mut().next.clone();
        slow2 = slow2.unwrap().borrow_mut().next.clone();
        if slow == slow2 {
            break;
        }
    }
    slow
}
#[test]
fn test() {
    // list1
    let list1_head = build_list(&[1,2,3,4,5,6,7], -1); // 1,2,6,11,3,4,5
    let res = find_ring_start(list1_head);
    // let reversed = crate::link::leecode_interview_02_07::find_intersect(list1_head, list2_head);
    print!("reversed: ");
    if res.is_none() {
        println!("None");
    } else {
        println!("{}", res.unwrap().borrow().val);
    }
}
