mod base;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val, left: None, right: None
        }))
    }

    fn build_from_vec(vec: Vec<i32>) -> Tree {
        if vec.is_empty() {
            return None;
        }
        let mut nodes = vec.iter()
            .map(|&v| TreeNode::new(v))
        .collect::<Vec<_>>();
        for i in 0..nodes.len() {
            if i * 2 + 1 < nodes.len() {
                nodes[i].borrow_mut().left = Some(nodes[i*2+1].clone());
            }
            if i * 2 + 2 < nodes.len() {
                nodes[i].borrow_mut().right = Some(nodes[i*2+2].clone());
            }
        }
        Some(nodes[0].clone())
    }

    fn tree_to_vec(root: Tree) -> Vec<Option<i32>> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(root);
        while let Some(node_opt) = queue.pop_front() {
            match node_opt {
                None => {
                    res.push(None);
                }
                Some(node) => {
                    let node_ref = node.borrow();
                    res.push(Some(node_ref.val));
                    queue.push_back(node_ref.left.clone());
                    queue.push_back(node_ref.right.clone());
                }
            }
        }
        while res.last() == Some(&None) {
            res.pop();
        }
        res
    }

    fn print_tree_vec(root: Tree) {
        let v = Self::tree_to_vec(root);
        println!("{:?}", v);
    }
}

#[test]
fn test_build_from_vec() {
    let vec = vec![1, 2, 3,4,5];
    let tree = TreeNode::build_from_vec(vec);
    println!("end");
    let vec1 = TreeNode::tree_to_vec(tree);
    println!("vec1: {:?}", vec1);
}