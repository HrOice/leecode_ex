// 给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。
//
// 考虑 nums 的唯一元素的数量为 k。去重后，返回唯一元素的数量 k。
//
// nums 的前 k 个元素应包含 排序后 的唯一数字。下标 k - 1 之后的剩余元素可以忽略。
//
// 判题标准:
//
// 系统会用下面的代码来测试你的题解:
//
// int[] nums = [...]; // 输入数组
// int[] expectedNums = [...]; // 长度正确的期望答案
//
// int k = removeDuplicates(nums); // 调用
//
// assert k == expectedNums.length;
// for (int i = 0; i < k; i++) {
//     assert nums[i] == expectedNums[i];
// }
// 如果所有断言都通过，那么您的题解将被 通过。
//
//
//
// 示例 1：
//
// 输入：nums = [1,1,2]
// 输出：2, nums = [1,2,_]
// 解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
// 示例 2：
//
// 输入：nums = [0,0,1,1,1,2,2,3,3,4]
// 输出：5, nums = [0,1,2,3,4,_,_,_,_,_]
// 解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。
//

// 双指针，当fast = slow时，fast前进，否则slow前进，并交换slow与fast
// [0..slow] 正确
// (slow..fast) 待处理
// [fast..end)待扫描
fn remove_duplicates_review(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut slow = 0;
    for fast in 1..nums.len() {
        let val = nums[fast];
        if nums[slow] == val {
            continue;
        }
        slow += 1;
        nums[slow] = val;
    }

    slow as i32 + 1
}

// [0..slow] 正确序列
// (slow..fast) 需要替换的区间
// [fast..end) 未扫描
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut slow = 0;
    let mut val = nums[slow];
    for i in 0..nums.len() {
        if nums[i] != val {
            slow += 1;
            nums[slow] = nums[i];
            val = nums[slow];
        }
    }
    slow as i32 + 1
}

#[test]
fn test_2() {
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let val = 3;
    let result = remove_duplicates_review(&mut nums);
    assert_eq!(result, 5);
    assert_eq!(2, remove_duplicates_review(&mut vec![1,1,2]));
    assert_eq!(1, remove_duplicates_review(&mut vec![1,1]));
    assert_eq!(2, remove_duplicates_review(&mut vec![1,2]));
    assert_eq!(0, remove_duplicates_review(&mut vec![]));
}