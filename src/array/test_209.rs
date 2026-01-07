// 给定一个含有 n 个正整数的数组和一个正整数 s ，
// 找出该数组中满足其和 ≥ s 的长度最小的 连续 子数组，并返回其长度。如果不存在符合条件的子数组，返回 0。
//
// 示例：
//
// 输入：s = 7, nums = [2,3,1,2,4,3]
// 输出：2
// 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
// 提示：
//
// 1 <= target <= 10^9
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^5

// 求和问题，暴力解法就是两次遍历，其实包含大量的重复计算 a + b + c, b + c + d
// 为了消除重复计算，就要把重复计算的值进行保存,可以看到中间相交的部分是可以处理的，
// 双指针拉绳，中间区域不断缩小求得最小长度

pub fn min_sub_array_len_review(target: i32, nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut slow = 0;
    let mut len = i32::MAX;
    for fast in 0..nums.len() {
        sum += nums[fast];
        while sum >= target {
            let new_len = (fast - slow + 1) as i32;
            if len > new_len {
                len = new_len
            }
            sum -= nums[slow];
            slow += 1;
        }
    }
    if len == i32::MAX {
        return 0;
    }
    len
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut j = 0;
    let mut sum = 0;
    let mut len = i32::max_value();
    for (pos, val) in nums.iter().enumerate() {
        sum += val;
        while sum >= target {
            let new_len = (pos -j + 1) as i32;
            if new_len < len {
                len = new_len;
            }
            sum -= nums[j];
            j += 1;
        }
    }
    if len == i32::max_value() {
        return 0;
    }
    len as i32
}

#[test]
fn test() {
    let nums = vec![2,3,1,2,4,3];
    let target = 7;
    let res = min_sub_array_len_review(target, nums);
    assert_eq!(res, 2);
}