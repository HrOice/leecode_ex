// 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a], nums[b], nums[c], nums[d]]
// （若两个四元组元素一一对应，则认为两个四元组重复）：
//
// 0 <= a, b, c, d < n
// a、b、c 和 d 互不相同
// nums[a] + nums[b] + nums[c] + nums[d] == target
// 你可以按 任意顺序 返回答案 。
//
//
//
// 示例 1：
//
// 输入：nums = [1,0,-1,0,-2,2], target = 0
// 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
// 示例 2：
//
// 输入：nums = [2,2,2,2,2], target = 8
// 输出：[[2,2,2,2]]

use crate::hash::lc_15::three_sum;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n < 4 {
        return vec![];
    }

    let mut nums = nums;
    nums.sort_unstable();

    let target = target as i64;
    let mut res = Vec::new();

    for i in 0..n - 3 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        for j in i + 1..n - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            let mut l = j + 1;
            let mut r = n - 1;

            while l < r {
                let sum = nums[i] as i64
                    + nums[j] as i64
                    + nums[l] as i64
                    + nums[r] as i64;

                if sum == target {
                    res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;

                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                } else if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
    }

    res
}

#[test]
fn test() {
    // let nums = vec![2,-3,0,-2,-5,-5,-4,1,2,-2,2,0,2,-4,5,5,-10];
    // let nums = vec![-1,0,1,2,-1,-4];
    // let nums = vec![1,0,-1,0,-2,2];
    let nums = vec![1000000000,1000000000,1000000000,1000000000];
    // let nums = vec![1,-2,-5,-4,-3,3,3,5];
    // let nums = vec![2,2,2,2,2];
    let res = four_sum(nums, -294967296);
    println!("{:?}", res);
}