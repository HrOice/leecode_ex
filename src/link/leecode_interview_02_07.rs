// 给你两个单链表的头节点 headA 和 headB ，请你找出并返回两个单链表相交的起始节点。如果两个链表没有交点，返回 null 。

// 输入：intersectVal = 2, listA = [0,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1
// 输出：Intersected at '2'
// 解释：相交节点的值为 2 （注意，如果两个链表相交则不能为 0）。
// 从各自的表头开始算起，链表 A 为 [0,9,1,2,4]，链表 B 为 [3,2,4]。
// 在 A 中，相交节点前有 3 个节点；在 B 中，相交节点前有 1 个节点。

// 输入：intersectVal = 8, listA = [4,1,8,4,5], listB = [5,0,1,8,4,5], skipA = 2, skipB = 3
// 输出：Intersected at '8'
// 解释：相交节点的值为 8 （注意，如果两个链表相交则不能为 0）。
// 从各自的表头开始算起，链表 A 为 [4,1,8,4,5]，链表 B 为 [5,0,1,8,4,5]。
// 在 A 中，相交节点前有 2 个节点；在 B 中，相交节点前有 3 个节点。

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::rc::Rc;

#[derive(Debug, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

pub(crate) type Link = Option<Rc<RefCell<ListNode>>>;

impl PartialEq for ListNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.next == other.next
    }
}

//
fn find_intersect(list_a: Link, list_b: Link) -> Link {
    if list_a.is_none() || list_b.is_none() {
        return None;
    }
    let mut cur_a = list_a.clone();
    let mut cur_b = list_b.clone();
    while cur_a.is_some() && cur_b.is_some() {
        cur_a = cur_a.unwrap().borrow().next.clone();
        cur_b = cur_b.unwrap().borrow().next.clone();
    }

    if cur_a.is_none() {
        cur_a = list_a;
    }
    if cur_b.is_none() {
        cur_b = list_b;
    }
    while cur_a.is_some() && cur_b.is_some() {
        cur_a = cur_a.unwrap().borrow().next.clone();
        cur_b = cur_b.unwrap().borrow().next.clone();
    }

    if cur_a.is_none() {
        cur_b
    } else {
        cur_a
    }
}
pub(crate) fn build_list(vals: &[i32]) -> (Link, Link) {
    let mut head: Link = None;
    let mut tail: Link = None;

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

        tail = Some(node);
    }

    (head, tail)
}
pub fn print_list(mut head: Link) {
    while let Some(node) = head {
        print!("{} ", node.borrow().val);
        head = node.borrow().next.clone();
    }
    println!();
}
#[test]
fn test() {
    // 公共相交部分
    let (intersect_head, intersect_tail) = build_list(&[3, 4, 5]);

    // list1
    let (list1_head, list1_tail) = build_list(&[1, 2,6,11]); // 1,2,6,11,3,4,5
    list1_tail.unwrap().borrow_mut().next = intersect_head.clone();

    // list2
    let (list2_head, list2_tail) = build_list(&[9, 8]); // 9,8,3,4,5
    list2_tail.unwrap().borrow_mut().next = intersect_head.clone();



    let reversed = find_intersect(list1_head, list2_head);
    print!("reversed: ");
    print_list(reversed);
}
