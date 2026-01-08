// 给你四个整数数组 nums1、nums2、nums3 和 nums4 ，数组长度都是 n ，请你计算有多少个元组 (i, j, k, l) 能满足：
//
// 0 <= i, j, k, l < n
// nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
//
//
// 示例 1：
//
// 输入：nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
// 输出：2
// 解释：
// 两个元组如下：
// 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
// 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0
// 示例 2：
//
// 输入：nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
// 输出：1
//
//
//   提示：
//
// n == nums1.length
// n == nums2.length
// n == nums3.length
// n == nums4.length
// 1 <= n <= 200
// -2^28 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 2^28

use crate::hash::lc_001::two_sum;

// 暴力需要O(n^4),解决重复计算能大大减少时间
// 4个数组，分成两份，恰好是两数之和的问题
// nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
// map<sum, group>
pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::<i32, i32>::new();
    let mut res = 0;
    for num in nums1 {
        for num2 in &nums2 {
            let sum = num + num2;
            map.entry(sum)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }
    for num in nums3 {
        for num2 in &nums4 {
            let sum = num + num2;
            if let Some(c1) = map.get(&-sum) {
                res += *c1;
            }
        }
    }
    res
}

#[test]
fn two_sum_test() {
    let nums1 = vec![1,2];
    let nums2 = vec![-2,-1];
    let nums3 = vec![-1,2];
    let nums4 = vec![0,2];

    let res = four_sum_count(nums1, nums2, nums3, nums4);
    assert_eq!(res, 2);
}