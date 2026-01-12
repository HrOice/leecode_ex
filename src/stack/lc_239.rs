// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
//
// 返回 滑动窗口中的最大值 。
//
//
//
// 示例 1：
//
// 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
// 输出：[3,3,5,5,6,7]
// 解释：
// 滑动窗口的位置                最大值
// ---------------               -----
// [1  3  -1] -3  5  3  6  7       3
//  1 [3  -1  -3] 5  3  6  7       3
//  1  3 [-1  -3  5] 3  6  7       5
//  1  3  -1 [-3  5  3] 6  7       5
//  1  3  -1  -3 [5  3  6] 7       6
//  1  3  -1  -3  5 [3  6  7]      7
// 示例 2：
//
// 输入：nums = [1], k = 1
// 输出：[1]
//
//
// 提示：
//
// 1 <= nums.length <= 105
// -104 <= nums[i] <= 104
// 1 <= k <= nums.length

use std::collections::VecDeque;

// [1,3,-1,-3,5,3,6,7]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut queue:VecDeque<usize> = VecDeque::with_capacity(k as usize); // 存下标，间接找值
    let mut res = vec![];
    for i in 0..nums.len() {
        let num = nums[i];
        if let Some(&first) = queue.front() {
            if i - first >= k as usize {
                queue.pop_front();
            }
        }
        while let Some(&back) = queue.back() {
            if nums[back] < num {
                queue.pop_back();
            } else {
                break;
            }
        }
        queue.push_back(i);
        if i + 1 >= k as usize {
            res.push(nums[*queue.front().unwrap()]);
        }
    }
    res
}

#[test]
fn test() {
    let s = max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3);
    assert_eq!(s, vec![3, 3,5,5,6,7]);
}