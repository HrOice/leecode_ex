use crate::link::{build_list, print_list, ListNode};
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
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    dummy_head.next = head;
    let mut fast = &dummy_head.clone();
    let mut slow = &mut dummy_head;

    for i in 0..n {
        fast = fast.next.as_ref().unwrap();
    }

    while fast.next.is_some() {
        fast = fast.next.as_ref().unwrap();
        slow = slow.next.as_mut().unwrap();
    }

    slow.next = slow.next.as_mut().unwrap().next.take();
    dummy_head.next
}

pub fn remove_nth_from_end1(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    dummy_head.next = head;
    let mut fast = &dummy_head.clone();
    let mut slow = &mut dummy_head;
    while n > 0 {
        fast = fast.next.as_ref().unwrap();
        n -= 1;
    }
    while fast.next.is_some() {
        fast = fast.next.as_ref().unwrap();
        slow = slow.next.as_mut().unwrap();
    }
    slow.next = slow.next.as_mut().unwrap().next.take();
    dummy_head.next
}

#[test]
fn test() {
    let head = build_list(&[1,2,3]);
    print!("original: ");
    print_list(head.clone());

    let reversed = remove_nth_from_end(head, 2);
    print!("reversed: ");
    print_list(reversed);
}
