///给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。
//
// 示例 1:
//
// 输入: nums = [-1,0,3,5,9,12], target = 9
// 输出: 4
// 解释: 9 出现在 nums 中并且下标为 4
// 示例 2:
//
// 输入: nums = [-1,0,3,5,9,12], target = 2
// 输出: -1
// 解释: 2 不存在 nums 中因此返回 -1
// 提示：
//
// 你可以假设 nums 中的所有元素是不重复的。
// n 将在 [1, 10000]之间。
// nums 的每个元素都将在 [-9999, 9999]之间。

fn search(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return mid as i32;
        } else if arr[mid] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    -1
}

fn search_open(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return mid as i32;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    -1
}

mod test {
    use crate::array::test_704::{search, search_open};

    #[test]
    fn test_1() {
        let nums = [-1,0,3,5,9,12];
        let res = search_open(&nums, 2);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_2() {
        let nums = [-1,0,3,5,9,12];
        let res = search_open(&nums, 9);
        assert_eq!(res, 4);
    }
}
