//给你一个非负整数 x ，计算并返回 x 的 算术平方根 。
//
// 由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。
//
// 注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。
//
//
//
// 示例 1：
//
// 输入：x = 4
// 输出：2
// 示例 2：
//
// 输入：x = 8
// 输出：2
// 解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。
//

// 1，2，3，4，5，6，7，8，9
// 2以上的，转换为i*i < n 的值
// 同样是要找到一个值，所以要把区间缩到1，while left <= right 
// while 是什么时候结束
// 1.left == right
// 2. left > right

// A A A A | B B B B B
// 当要找到这个分界点时， 比如left = right = B
// mid = B， （假设找比target小的）如果target < mid，right左移，此时right就是目标分界。
// mid = A，如果target > mid, left右移，此时还是right右侧分界点
//  （假设找比target大的），那么就是取left作为分界。

pub fn my_sqrt_review(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
    let mut left = 1;
    let mut right = x / 2;
    while left <= right {
        let mid = left + (right - left) / 2;
        if mid > x / mid{
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    left
}

pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
    let mut left = 1;
    let mut right = x / 2;
    while left <= right {
        let mid = left + (right - left) / 2;
        if mid > x / mid {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    right
}

#[test]
fn test_1() {
    let x = 6;
    let res = my_sqrt_review(x);
    assert_eq!(res, 2);
}