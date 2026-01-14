// 给你一个整数数组 nums 和一个整数 k ，请你返回其中出现频率前 k 高的元素。你可以按 任意顺序 返回答案。
//
//
//
// 示例 1：
//
// 输入：nums = [1,1,1,2,2,3], k = 2
//
// 输出：[1,2]
//
// 示例 2：
//
// 输入：nums = [1], k = 1
//
// 输出：[1]
//
// 示例 3：
//
// 输入：nums = [1,2,1,2,1,2,3,1,3,2], k = 2
//
// 输出：[1,2]
//
//
//
// 提示：
//
// 1 <= nums.length <= 105
// -10^4 <= nums[i] <= 10^4
// k 的取值范围是 [1, 数组中不相同的元素的个数]
// 题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的
//
//
// 进阶：你所设计算法的时间复杂度 必须 优于 O(n log n) ，其中 n 是数组大小。

use std::collections::HashMap;

// 桶排序，先统计，频率转换桶，也就是数组元素索引，每个频率放对应的数字集合
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let len = nums.len();
    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut buckets = vec![vec![]; len + 1];
    for (num, freq) in map {
        buckets[freq].push(num);
    }
    let mut res = vec![];
    for bucket in buckets.iter().rev() {
        for num in bucket {
            if res.len() < k as usize {
                res.push(*num);
            } else {
                return res;
            }
        }
    }
    res
}

// 1 2 4 6 3 5 7
//
#[derive(Debug)]
struct Heap {
    heap: Vec<i32>,
}

// 逐级上浮
// 完全二叉树的特性
// 当前节点i， 父节点 (i-1) / 2 , 子节点(2i - 1,2i),
// j层有 2^j 个节点
//

impl Heap {
    fn new() -> Self {
        Heap { heap: vec![] }
    }



    fn push(&mut self, x: i32) {
        self.heap.push(x);
        self.sift_up(self.heap.len() - 1);
    }

    // i 当前节点
    fn sift_up(&mut self, mut i: usize) {
        while i > 0  {
            let parent = (i - 1) / 2;
            if self.heap[parent] > self.heap[i] {
                break;
            }
            self.heap.swap(i, parent);
            i = parent;
        }
    }
    fn pop(&mut self) -> Option<i32> {
        if self.heap.len() == 0 {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let res = self.heap.pop();
        self.sift_down(0);
        res
    }

    fn sift_down(&mut self, mut i: usize) {
        loop {
            let mut largest = i;
            let left = i*2 + 1;
            let right = i*2 + 2;
            if left < self.heap.len() && self.heap[largest] < self.heap[left] {
                largest = left;
            }
            if right < self.heap.len() &&self.heap[largest] < self.heap[right] {
                largest = right;
            }
            if largest == i {
                break;
            }
            self.heap.swap(i, largest);
            i = largest;
        }
    }
    fn peek(&self) -> i32 {
        self.heap[0]
    }
}

// 由下自上寻找大值
// 找到倒数第一层,节点 n / 2 - 1
fn heapify(nums: &mut Vec<i32>) {
    if nums.len() < 1 {
        return;
    }
    for i in (0..=nums.len()/ 2 - 1).rev() {
        sift_down(nums, i);
    }
}
fn sift_down(nums: &mut Vec<i32>, mut i: usize) {
    loop {
        let mut largest = i;
        let left = i * 2 + 1;
        let right = i * 2 + 2;
        if right < nums.len() && nums[largest] < nums[right] {
            largest = right;
        }
        if left < nums.len() && nums[largest] < nums[left] {
            largest = left;
        }
        if largest == i {
            return;
        }
        nums.swap(i, largest);
        i = largest;
    }
}

#[test]
fn test_heapify() {
    let mut nums = vec![1, 2, 4, 6, 3, 5, 7];
    heapify(&mut nums);
    println!("{:?}", nums);
}

#[test]
fn test_heap() {
    let mut h = Heap::new();
    let nums = vec![1, 2, 4, 6, 3, 5, 7];
    for i in nums.clone() {
        h.push(i);
    }
    println!("{:?}", h);
    for i in nums {
        println!("{:?}", h.pop());
    }
}

#[test]
fn test() {
    let res = top_k_frequent(vec![1,1,1,2,2,3], 2);
    println!("{:?}", res);
}