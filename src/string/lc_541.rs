// 给定一个字符串 s 和一个整数 k，从字符串开头算起，每计数至 2k 个字符，就反转这 2k 字符中的前 k 个字符。
//
// 如果剩余字符少于 k 个，则将剩余字符全部反转。
// 如果剩余字符小于 2k 但大于或等于 k 个，则反转前 k 个字符，其余字符保持原样。
//
//
// 示例 1：
//
// 输入：s = "abcdefg", k = 2
// 输出："bacdfeg"
// 示例 2：
//
// 输入：s = "abcd", k = 2
// 输出："bacd"
//
//
// 提示：
//
// 1 <= s.length <= 10^4
// s 仅由小写英文组成
// 1 <= k <= 10^4
pub fn reverse_str(s: String, k: i32) -> String {
    let mut s = s.into_bytes();
    let k = k as usize;
    let len = s.len();
    for i in (0..len).step_by(2 * k) {
        if i + k < len {
            reverse(&mut s, i, i+k -1);
        } else {
            reverse(&mut s, i, len - 1);
        }
    }
    String::from_utf8(s).unwrap()
}
pub fn reverse(s: &mut Vec<u8>, mut slow: usize, mut fast: usize) {
    while slow < fast {
        let tmp = s[fast];
        s[fast] = s[slow];
        s[slow] = tmp;
        slow += 1;
        fast -= 1;
    }
}

#[test]
fn test() {
    // let res = reverse_str("abcdefg".to_string(), 2);
    // assert_eq!(res, "bacdfeg");
    // assert_eq!(reverse_str("abcd".to_string(), 4), "dcba");
    assert_eq!(reverse_str("abcdefg".to_string(), 8), "gfedcba");
}
