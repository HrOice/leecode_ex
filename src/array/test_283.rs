// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
//
// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
//
//
//
// 示例 1:
//
// 输入: nums = [0,1,0,3,12]
// 输出: [1,3,12,0,0]
// 示例 2:
//
// 输入: nums = [0]
// 输出: [0]


pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut slow = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            let slow_num = nums[slow];
            nums[slow] = nums[i];
            nums[i] = slow_num;
            slow += 1;
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![0,1,0,3,12];
    move_zeroes(&mut nums);
    assert_eq!(nums, vec![1,3,12,0,0]);
}
