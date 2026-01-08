
// 编写一个算法来判断一个数 n 是不是快乐数。
//
// 「快乐数」 定义为：
//
// 对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
// 然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
// 如果这个过程 结果为 1，那么这个数就是快乐数。
// 如果 n 是 快乐数 就返回 true ；不是，则返回 false 。
//
//
//
// 示例 1：
//
// 输入：n = 19
// 输出：true
// 解释：
// 1^2 + 9^2 = 82
// 8^2 + 2^2 = 68
// 6^2 + 8^2 = 100
// 1^2 + 0^2 + 0^2 = 1
// 示例 2：
//
// 输入：n = 2
// 输出：false
//
//
// 提示：
//
// 1 <= n <= 231 - 1
pub fn is_happy(n: i32) -> bool {
    use std::collections::HashSet;
    if n == 1 {
        return true;
    }
    let mut sum_set = HashSet::new();
    let mut sum = 0;
    let mut n = n;

    while n > 1 {
        let mut tail = n % 10;
        let mut prefix = n / 10;
        while prefix > 0 || tail > 0{
            sum += tail * tail;
            tail = prefix % 10;
            prefix /= 10;
        }
        n = sum;
        if (!sum_set.insert(sum)) {
            return false;
        }
        if sum == 1 {
            return true;
        }
        sum = 0;
    }

    sum == 1
}

#[test]
fn test_is_happy() {
    assert!(is_happy(19));
    assert!(!is_happy(2));
}