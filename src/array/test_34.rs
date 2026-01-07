// 给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。
//
// 如果数组中不存在目标值 target，返回 [-1, -1]。
//
// 你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。
//
//
//
// 示例 1：
//
// 输入：nums = [5,7,7,8,8,10], target = 8
// 输出：[3,4]
// 示例 2：
//
// 输入：nums = [5,7,7,8,8,10], target = 6
// 输出：[-1,-1]
// 示例 3：
//
// 输入：nums = [], target = 0
// 输出：[-1,-1]


// fast 前进，遇到val = fast，将slow赋值，
// fast 继续前进，遇到val != fast，停止
// 否则 -1 这是暴力搜索，需要O(n)
// 要达到O(logn),需要二分搜索
// 二分搜索，
pub fn search_range_review(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }
    let lower = lower_bound_review(&nums, target);
    if lower == -1 {
        return vec![-1, -1];
    }
    let upper = upper_bound_review(&nums, target);
    vec![lower, upper]

}
// 1,2,3,3,4,5,6,6,6,7,7,7,8
fn lower_bound_review(nums: &Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        let mid_val = nums[mid];
        if mid_val < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if nums[left] != target {
        return -1;
    }
    right as i32
}
fn upper_bound_review(nums: &Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    if nums.len() == 1 && target == nums[0] {
        return 0;
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        let mid_val = nums[mid];
        if mid_val <= target {
            left = mid + 1
        } else {
            right = mid;
        }
    }
    if left == nums.len() -1 && target == nums[left] {
        return left as i32;
    }
    left as i32 - 1
}

#[test]
fn test_bin_search_review() {
    // assert_eq!(lower_bound_review(&vec![1, 2, 3,3,3, 4, 5], 3), 2);
    // assert_eq!(upper_bound_review(&vec![1, 2, 3,3,3, 4, 5], 3), 4);
    // assert_eq!(search_range_review(vec![1, 2, 3,3,3, 4, 5], 3), [2,4]);
    // assert_eq!(search_range_review(vec![5,7,7,8,8,10], 8), [3,4]);
    // assert_eq!(search_range_review(vec![5,7,7,8,8,10], 6), [-1, -1]);
    // assert_eq!(search_range_review(vec![1], 1), [0,0]);
    assert_eq!(search_range_review(vec![2,2], 2), [0,1]);
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1]
    }
    let left = lower_bound(&nums, target);
    if left == 0 && nums[0] != target {
        return [-1, -1].to_vec();
    }

    let right = upper_bound(&nums, target);
    [left as i32, right as i32].to_vec()
}

fn lower_bound(arr: &Vec<i32>, target: i32) -> i32 {
    if arr.len() == 1 {
        if arr[0] == target {
            return 0;
        } else {
            return -1;
        }
    }
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if left <arr.len() && arr[left] == target {
        left as i32
    } else if left - 1 < arr.len() && arr[left - 1] == target {
        left as i32 -1
    } else {
        -1
    }
}

fn upper_bound(arr: &Vec<i32>, target: i32) -> i32 {
    if arr.len() == 1 {
        if arr[0] == target {
            return 0;
        } else {
            return -1;
        }
    }
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left <arr.len() && arr[left] == target {
        left as i32
    } else if left - 1 < arr.len() && arr[left - 1] == target {
        left as i32 -1
    } else {
        -1
    }
}

#[test]
fn test() {
    let nums = vec![2,2];
    let target = 2;
    let res = search_range(nums, target);
    assert_eq!(res, [-1, -1]);
}