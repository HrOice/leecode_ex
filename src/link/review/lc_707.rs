// 你可以选择使用单链表或者双链表，设计并实现自己的链表。
//
// 单链表中的节点应该具备两个属性：val 和 next 。val 是当前节点的值，next 是指向下一个节点的指针/引用。
//
// 如果是双向链表，则还需要属性 prev 以指示链表中的上一个节点。假设链表中的所有节点下标从 0 开始。
//
// 实现 MyLinkedList 类：
//
// MyLinkedList() 初始化 MyLinkedList 对象。
// int get(int index) 获取链表中下标为 index 的节点的值。如果下标无效，则返回 -1 。
// void addAtHead(int val) 将一个值为 val 的节点插入到链表中第一个元素之前。在插入完成后，新节点会成为链表的第一个节点。
// void addAtTail(int val) 将一个值为 val 的节点追加到链表中作为链表的最后一个元素。
// void addAtIndex(int index, int val) 将一个值为 val 的节点插入到链表中下标为 index 的节点之前。如果 index 等于链表的长度，那么该节点会被追加到链表的末尾。如果 index 比长度更大，该节点将 不会插入 到链表中。
// void deleteAtIndex(int index) 如果下标有效，则删除链表中下标为 index 的节点。
//
//
// 示例：
//
// 输入
// ["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
// [[], [1], [3], [1, 2], [1], [1], [1]]
// 输出
// [null, null, null, null, 2, null, 3]
//
// 解释
// MyLinkedList myLinkedList = new MyLinkedList();
// myLinkedList.addAtHead(1);
// myLinkedList.addAtTail(3);
// myLinkedList.addAtIndex(1, 2);    // 链表变为 1->2->3
// myLinkedList.get(1);              // 返回 2
// myLinkedList.deleteAtIndex(1);    // 现在，链表变为 1->3
// myLinkedList.get(1);              // 返回 3
//

struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
}

impl MyLinkedList {

    fn new() -> Self {
        MyLinkedList { val: -1, next: None }
    }

    // 获取链表中下标为 index 的节点的值。如果下标无效，则返回 -1 。
    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut cur = self.next.as_ref();
        let mut idx = index;
        while idx > 0 {
            match cur {
                None => {
                    return -1;
                }
                Some(node) => {
                    cur = node.next.as_ref();
                }
            }

            idx -= 1;
        }
        cur.map_or(-1, |node| node.val)
    }

    // 将一个值为 val 的节点插入到链表中第一个元素之前。在插入完成后，新节点会成为链表的第一个节点。
    fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList {
           val, next: self.next.take(),
        });
        self.next = Some(new_node);
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut cur = self;
        while cur.next.is_some() {
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = Some(Box::new(MyLinkedList {
            val, next: None
        }))
    }

    // 将一个值为 val 的节点插入到链表中下标为 index 的节点之前。如果 index 等于链表的长度，
    // 那么该节点会被追加到链表的末尾。如果 index 比长度更大，该节点将 不会插入 到链表中。
    fn add_at_index(&mut self, index: i32, val: i32) {
        // 插入，需要前置节点
        let mut cur = self;
        let mut idx = index;
        while idx > 0 {
            if cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
                idx -= 1;
            } else {
                return;
            }
        }
        let new_node = Box::new(MyLinkedList {
            val, next: cur.next.take(),
        });
        cur.next = Some(new_node);
    }

    // void deleteAtIndex(int index) 如果下标有效，则删除链表中下标为 index 的节点。
    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        let mut cur = self;
        let mut idx = index;
        while idx > 0 {
            match cur.next.as_mut() {
                None => {return;}
                Some(node) => {
                    cur = node;
                }
            }
            idx -= 1;
        }
        if let Some(node) = cur.next.take() {
            cur.next = node.next;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::link::{build_list, print_list};
    use super::*;

    fn list_to_vec(mut head: &Option<Box<MyLinkedList>>) -> Vec<i32> {
        let mut res = vec![];
        while let Some(node) = head {
            res.push(node.val);
            head = &node.next;
        }
        res
    }

    #[test]
    fn test_get() {
        let mut list = Box::new(MyLinkedList::new());
        list.add_at_head(1);
        list.add_at_head(0);
        // -1 0 1

        assert_eq!(list.get(0), 0);
        assert_eq!(list.get(1), 1);
        assert_eq!(list.get(-1), -1);
        assert_eq!(list.get(11), -1);
    }

    #[test]
    fn test_add_at_head() {
        let mut list = Box::new(MyLinkedList::new());

        list.add_at_head(1);
        list.add_at_head(0);
        assert_eq!(list_to_vec(&Some(list)), vec![-1,0,1]);
    }
    //
    #[test]
    fn test_add_at_tail() {
        let mut list = Box::new(MyLinkedList::new());

        list.add_at_head(1);
        list.add_at_head(0);
        list.add_at_tail(4); // 0 1 4

        list.add_at_tail(5);
        assert_eq!(list_to_vec(&Some(list)), vec![-1, 0 ,1, 4, 5]);
    }
    //
    #[test]
    fn test_add_at_index_middle() {
        let mut list = Box::new(MyLinkedList::new());

        list.add_at_head(1);
        list.add_at_head(0);
        list.add_at_tail(4); // 0 1 4

        list.add_at_tail(5);
        list.add_at_index(2, 3); // 0 1 4 5
        assert_eq!(list_to_vec(&Some(list)), vec![-1, 0, 1, 3,4,5]);
    }
    //
    #[test]
    fn test_add_at_index_head_and_tail() {
        let mut list = Box::new(MyLinkedList::new());

        list.add_at_head(1);
        list.add_at_head(0);
        list.add_at_tail(4); // 0 1 4

        list.add_at_tail(5);
        list.add_at_index(0, 10); // 0 1 4 5
        list.add_at_index(5, 20); // 0 1 4 5
        assert_eq!(list_to_vec(&Some(list)), vec![-1, 10,0,1,4,5,20]);
    }
    //
    #[test]
    fn test_add_at_index_invalid() {
        let mut list = Box::new(MyLinkedList::new());

        list.add_at_head(1);
        list.add_at_head(0);
        list.add_at_tail(4); // 0 1 4

        list.add_at_tail(5);
        list.add_at_index(10, 99); // 无效
        assert_eq!(list_to_vec(&Some(list)), vec![-1,0,1,4,5]);
    }
    //
    #[test]
    fn test_delete_at_index_middle() {
        let mut list = Box::new(MyLinkedList::new());

        list.add_at_head(1);
        list.add_at_head(0);
        list.add_at_tail(4); // 0 1 4

        list.add_at_tail(5);
        list.delete_at_index(2);
        assert_eq!(list_to_vec(&Some(list)), vec![-1,0,1,5]);

    }
    //
    #[test]
    fn test_delete_at_index_head_and_tail() {
        let mut list = Box::new(MyLinkedList::new());

        list.add_at_head(1);
        list.add_at_head(0);
        list.add_at_tail(4); // 0 1 4

        list.add_at_tail(5);
        list.delete_at_index(0);
        assert_eq!(list_to_vec(&Some(list)), vec![-1,1,4,5]);
    }
    //
    #[test]
    fn test_delete_at_index_invalid() {
        let mut list = Box::new(MyLinkedList::new());

        list.delete_at_index(5);
        assert_eq!(list_to_vec(&Some(list)), vec![-1]);
    }


}
