
mod leecode_203;
mod leecode_707;
mod leecode_206;
mod leecode_24;
mod leecode_19;
mod leecode_interview_02_07;
mod lc_142;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn build_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in nums.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

pub fn build_list_for_tail(nums: &[i32]) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut head = None;
    let mut tail = None;
    for &v in nums.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        if tail.is_none() {
            tail = Some(node.clone());
        }
        node.next = head;
        head = Some(node);
    }
    (head, tail)
}

pub fn print_list(mut head: Option<Box<ListNode>>) {
    while let Some(node) = head {
        print!("{} ", node.val);
        head = node.next;
    }
    println!();
}