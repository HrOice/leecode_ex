use std::collections::HashSet;

// 给定两个数组 nums1 和 nums2 ，返回 它们的 交集 。输出结果中的每个元素一定是 唯一 的。我们可以 不考虑输出结果的顺序 。
//
//
//
// 示例 1：
//
// 输入：nums1 = [1,2,2,1], nums2 = [2,2]
// 输出：[2]
// 示例 2：
//
// 输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// 输出：[9,4]
// 解释：[4,9] 也是可通过的
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result_set = HashSet::new();
    for num in nums1 {
        result_set.insert(num);
    }
    let mut result = HashSet::new();
    for num in nums2 {
        if result_set.contains(&num) {
            result.insert(num);
        }
    }
    result.into_iter().collect()
}

// 另外算法：排序+ 双指针 时间：O(n log n + m log m) 空间：O(1)（不算排序栈）
// 位图，不引入hash结构，O(m+n) 空间：O(range)