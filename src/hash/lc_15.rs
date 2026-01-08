// 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
//
// 注意：答案中不可以包含重复的三元组。
//
// 示例 1：
//
// 输入：nums = [-1,0,1,2,-1,-4]
// 输出：[[-1,-1,2],[-1,0,1]]
// 解释：
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
// 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
// 注意，输出的顺序和三元组的顺序并不重要。
// 示例 2：
//
// 输入：nums = [0,1,1]
// 输出：[]
// 解释：唯一可能的三元组和不为 0 。
// 示例 3：
//
// 输入：nums = [0,0,0]
// 输出：[[0,0,0]]
// 解释：唯一可能的三元组和为 0 。

use std::collections::HashMap;
// O(n^3) -> n^2
// hash法，去重有麻烦
// [-1,0,1,2,-1,-4]
// 双指针法
// [-4,-1,-1,0,1,2]
// 一个指针固定，两个指针移动，变成两数之和
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    println!("nums: {:?}", nums);
    let mut res = Vec::new();
    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        if nums[i] > 0 {
            break;
        }
        // 求两数之和 = -nums[i]
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        let target = - nums[i];
        while left < right {
            if nums[left] + nums[i] > 0 {
                break;
            }
            if left - 1> i && nums[left] == nums[left - 1] {
                left += 1;
                continue;
            }
            if right < nums.len() - 1 &&  nums[right] == nums[right + 1] {
                right -= 1;
                continue;
            }
            if nums[left] + nums[right] == target {
                res.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;
            } else if nums[left] + nums[right] > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
    }
    res
}



#[test]
fn test() {
    // let nums = vec![2,-3,0,-2,-5,-5,-4,1,2,-2,2,0,2,-4,5,5,-10];
    let nums = vec![-1,0,1,2,-1,-4];
    let res = three_sum(nums);
    println!("{:?}", res);
}