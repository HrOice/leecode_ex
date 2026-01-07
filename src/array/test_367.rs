// 给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，否则返回 false 。
//
// 完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成某个整数和自身的乘积。
//
// 不能使用任何内置的库函数，如  sqrt 。
//
//
//
// 示例 1：
//
// 输入：num = 16
// 输出：true
// 解释：返回 true ，因为 4 * 4 = 16 且 4 是一个整数。
// 示例 2：
//
// 输入：num = 14
// 输出：false
// 解释：返回 false ，因为 3.742 * 3.742 = 14 但 3.742 不是一个整数。
//

// 同69，查唯一值
pub fn is_perfect_square_review(num: i32) -> bool {
    if num < 2 {
        return true;
    }
    let mut left = 2;
    let mut right = num / 2;
    while left <= right { // 需要计算最后一个单独的mid
        let mid = left + (right - left) / 2;
        if mid * mid == num {
            return true;
        } else if mid * mid < num {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}

pub fn is_perfect_square(num: i32) -> bool {
    if num < 2 {
        return true;
    }
    let mut left = 1;
    let mut right = (num + 1)/ 2;
    while left <= right {
        let mid = left + (right - left) / 2;
        if mid * mid == num {
            return true;
        } else if mid > num / mid {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    false
}

#[test]
fn test_1() {
    let x = 14;
    let res = is_perfect_square_review(16);
    assert_eq!(res, true);
}