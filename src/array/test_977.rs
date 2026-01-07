// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
//
//
//
// 示例 1：
//
// 输入：nums = [-4,-1,0,3,10]
// 输出：[0,1,9,16,100]
// 解释：平方后，数组变为 [16,1,0,9,100]
// 排序后，数组变为 [0,1,9,16,100]
// 示例 2：
//
// 输入：nums = [-7,-3,2,3,11]
// 输出：[4,9,9,49,121]
//
//
// 提示：
//
// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums 已按 非递减顺序 排序

pub fn sorted_squares_review(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut idx = nums.len() - 1;
    while idx >= 0 {
        let n_right = nums[right] * nums[right];
        let n_left = nums[left] * nums[left];
        if n_right > n_left {
            res[idx] = n_right;
            right -= 1;
        } else {
            res[idx] = n_left;
            left += 1;
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    res
}
#[test]
fn test_review() {
    let nums = vec![1,2];
    let res = sorted_squares_review(nums);
    assert_eq!(res, vec![1,4]);
}


// 0左侧数字的绝对值应插入到右侧的什么地方
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut res = vec![0; nums.len()];
    let mut idx = nums.len() - 1;
    while left <= right {
        let r = nums[right];
        let r2 = r * r;
        let l = nums[left];
        let l2 = l * l;
        if l2 < r2 {
            res[idx] = r2;
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            left += 1;
            res[idx] = l2;
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    res
}

#[test]
fn test() {
    let nums = vec![1,2];
    let res = sorted_squares_review(nums);
    assert_eq!(res, vec![1,4]);
}

