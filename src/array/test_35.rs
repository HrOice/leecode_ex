// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
//
// 请必须使用时间复杂度为 O(log n) 的算法。
//
//
//
// 示例 1:
//
// 输入: nums = [1,3,5,6], target = 5
// 输出: 2
// 示例 2:
//
// 输入: nums = [1,3,5,6], target = 2
// 输出: 1
// 示例 3:
//
// 输入: nums = [1,3,5,6], target = 7
// 输出: 4
//
//
// 提示:
//
// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums 为 无重复元素 的 升序 排列数组
// -104 <= target <= 104

use std::cmp::Ordering;

fn search(arr: &[i32], target: i32) -> usize {
    let (mut left,mut right) = (0, arr.len() - 1);
    while left <= right {
        let mid = (left + right) / 2;
        match arr[mid].cmp(&target) {
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Equal => {
                return mid;
            }
            Ordering::Greater => {
                right = mid - 1;
            }
        }
    }
    right + 1
}

fn search_open(arr: &[i32], target: i32) -> usize {
    let (mut left,mut right) = (0, arr.len() );
    while left < right {
        let mid = (left + right) / 2;
        match arr[mid].cmp(&target) {
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Equal => {
                return mid;
            }
            Ordering::Greater => {
                right = mid;
            }
        }
    }
    right
}

mod test {
    use crate::array::test_35::{search, search_open};

    #[test]
    fn test_1() {
        let nums = [1,3,5,6];
        let res = search_open(&nums, 2);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_2() {
        let nums = [1,3,5,6];
        let res = search_open(&nums, 7);
        assert_eq!(res, 4);
    }
}