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


// 快慢步，在环内一定相遇
// a b c d e f g h d
// a ..x..d ..y m ..z d

// 假设从头结点到环形入口节点 的节点数为x。 环形入口节点到 fast指针与slow指针相遇节点 节点数为y。 从相遇节点 再到环形入口节点节点数为 z。
// 相遇时：slow: x + y   fast: x+y + n(y+z)
// 2*(x+y) = x+y + n(y+z)
// x+y = ny+nz
// x = (n-1)y + nz
// x = (n-1)(y+z) + z
// n = 1时 x= z，也就是说明，slow再走z，就是环起点


fn find_ring_start(head: crate::link::leecode_interview_02_07::Link) -> crate::link::leecode_interview_02_07::Link {
    let mut slow = head.clone();
    let mut fast = head.clone();
    while let (Some(slow_node), Some(fast_node)) = (slow.clone(), fast.clone()) {
        slow = slow_node.borrow_mut().next.clone();
        fast = fast_node.borrow_mut().next.clone();
        if fast.is_none() {
            return None;
        }
        fast = fast_node.borrow_mut().next.clone();
        if slow == fast {
            break;
        }
    }
    if slow.is_none() || fast.is_none() {
        return None;
    }
    let mut slow2 = head.clone();
    while slow != slow2 {
        slow = slow.unwrap().borrow().next.clone();
        slow2 = slow2.unwrap().borrow().next.clone();
    }
    slow
}