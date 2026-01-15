// 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
//
//给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
//
// 进阶：你能尝试使用一趟扫描实现吗？
//
// 示例 1：
//
//
// 输入：head = [1,2,3,4,5], n = 2
// 输出：[1,2,3,5]
// 示例 2：
//
// 输入：head = [1], n = 1
// 输出：[]
// 示例 3：
//
// 输入：head = [1,2], n = 1
// 输出：[1]
//

// 双指针，fast - slow = n 时，一起前进，直到fast到达终点
// dummy = fast = slow -> a -> b -> c -> d -> None

use crate::link::{build_list, print_list, ListNode};

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut i = 0;
    let mut dummy_head = Box::new(ListNode { val: 0, next: head });
    let mut cur = &mut dummy_head;
    while let Some(node) = cur.next.as_mut() {
        cur = node;
        i += 1;
    }
    cur = &mut dummy_head;
    while let Some(node) = cur.next.take() {
        if i == n {
            cur.next = node.next;
            break;
        }
        cur.next = Some(node);
        cur = cur.next.as_mut().unwrap();
        i -= 1;
    }
    dummy_head.next
}
//
// pub fn remove_nth_from_end1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//     let mut dummy = Box::new(ListNode {
//         val: 0,
//         next: head,
//     });
//
//     // fast：不可变引用
//     let mut fast = dummy.next.as_ref();
//     for _ in 0..n {
//         fast = fast.unwrap().next.as_ref();
//     }
//
//     // slow：可变引用
//     let mut slow = &mut dummy;
//
//     // ⚠️ 关键：不能用 while let
//     while fast.is_some() {
//         fast = fast.unwrap().next.as_ref();
//         slow = slow.next.as_mut().unwrap();
//     }
//
//     // 删除 slow.next
//     let next = slow.next.as_mut().unwrap().next.take();
//     slow.next = next;
//
//     dummy.next
// }
#[test]
fn test() {
    let head = build_list(&[1,2]);
    print!("original: ");
    print_list(head.clone());

    let reversed = remove_nth_from_end(head, 1);
    print!("reversed: ");
    print_list(reversed);
}