use crate::tree::{Tree, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
enum Order {
    Pre,
    In,
    Post,
}
impl TreeNode {
    fn preorder(root: &Tree) -> Vec<i32> {
        fn dfs(node: &Tree, vec: &mut Vec<i32>) {
            if let Some(rc) = node {
                let node = rc.borrow();
                vec.push(node.val);
                dfs(&node.left, vec);
                dfs(&node.right, vec);
            }
        }
        let mut vec = Vec::new();

        dfs(root, &mut vec);
        vec
    }



    fn unified_traversal(root: &Tree, order: Order) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack: Vec<(Rc<RefCell<TreeNode>>, bool)> = Vec::new();

        if let Some(rc) = root {
            stack.push((rc.clone(), false));
        }

        while let Some((node, visited)) = stack.pop() {
            if visited {
                res.push(node.borrow().val);
                continue;
            }

            let n = node.borrow();

            match order {
                Order::Pre => {
                    // 根 左 右
                    stack.push((node.clone(), true));
                    if let Some(r) = &n.right {
                        stack.push((r.clone(), false));
                    }
                    if let Some(l) = &n.left {
                        stack.push((l.clone(), false));
                    }
                }
                Order::In => {
                    // 左 根 右
                    if let Some(r) = &n.right {
                        stack.push((r.clone(), false));
                    }
                    stack.push((node.clone(), true));
                    if let Some(l) = &n.left {
                        stack.push((l.clone(), false));
                    }
                }
                Order::Post => {
                    // 左 右 根
                    stack.push((node.clone(), true));
                    if let Some(r) = &n.right {
                        stack.push((r.clone(), false));
                    }
                    if let Some(l) = &n.left {
                        stack.push((l.clone(), false));
                    }
                }
            }
        }

        res
    }

    fn inorder(root: &Tree) -> Vec<i32> {
        fn dfs(node: &Tree, vec: &mut Vec<i32>) {
            if let Some(rc) = node {
                let node = rc.borrow();
                dfs(&node.left, vec);
                vec.push(node.val);
                dfs(&node.right, vec);
            }
        }
        let mut vec = Vec::new();

        dfs(root, &mut vec);
        vec
    }

    fn postorder(root: &Tree) -> Vec<i32> {
        fn dfs(node: &Tree, vec: &mut Vec<i32>) {
            if let Some(rc) = node {
                let node = rc.borrow();
                dfs(&node.left, vec);
                dfs(&node.right, vec);
                vec.push(node.val);
            }
        }
        let mut vec = Vec::new();

        dfs(root, &mut vec);
        vec
    }

    fn iter_preorder(root: &Tree) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut res = Vec::new();
        queue.push_front(root.clone());
        while let Some(node_op) = queue.pop_front() {
            if let Some(rc) = node_op {
                let node = rc.borrow();
                res.push(node.val);
                queue.push_front(rc.borrow().right.clone());
                queue.push_front(rc.borrow().left.clone())
            }
        }
        res
    }

    fn iter_inorder(root: &Tree) -> Vec<i32> {
        let mut queue = Vec::new();
        let mut res = Vec::new();
        let mut node = root.clone();
        while node.is_some() || !queue.is_empty() {
            while let Some(rc) = node {
                node = rc.borrow().left.clone();
                queue.push(rc);
            }
            if let Some(rc) = queue.pop() {
                res.push(rc.borrow().val);
                node = rc.borrow().right.clone();
            }
        }
        res
    }

    fn iter_postorder(root: &Tree) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut res = Vec::new();
        queue.push_front(root.clone());
        while let Some(node_op) = queue.pop_front() {
            if let Some(rc) = node_op {
                let node = rc.borrow();
                res.push(node.val);
                queue.push_front(rc.borrow().left.clone());
                queue.push_front(rc.borrow().right.clone());
            }
        }
        res.reverse();
        res
    }

}

#[test]
fn test_iter() {
    let vec = (0..8).collect::<Vec<i32>>();
    let tree = TreeNode::build_from_vec(vec);
    let travel_middle = TreeNode::iter_preorder(&tree.clone());
    println!("{:?}", travel_middle);
    let travel_middle = TreeNode::iter_inorder(&tree.clone());
    println!("{:?}", travel_middle);
    let travel_middle = TreeNode::iter_postorder(&tree.clone());
    println!("{:?}", travel_middle);
}

#[test]
fn test_travel_front() {
    let vec = (0..8).collect::<Vec<i32>>();
    let tree = TreeNode::build_from_vec(vec);
    TreeNode::print_tree_vec(tree.clone());
    let travel_front = TreeNode::preorder(&tree.clone());
    println!("{:?}", travel_front);
    let travel_middle = TreeNode::inorder(&tree.clone());
    println!("{:?}", travel_middle);
    let travel_middle = TreeNode::postorder(&tree.clone());
    println!("{:?}", travel_middle);
    let layer = TreeNode::tree_to_vec(tree.clone());
    println!("{:?}", layer);
}
