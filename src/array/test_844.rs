// 给定 s 和 t 两个字符串，当它们分别被输入到空白的文本编辑器后，如果两者相等，返回 true 。# 代表退格字符。
//
// 注意：如果对空文本输入退格字符，文本继续为空。
//
//
//
// 示例 1：
//
// 输入：s = "ab#c", t = "ad#c"
// 输出：true
// 解释：s 和 t 都会变成 "ac"。
// 示例 2：
//
// 输入：s = "ab##", t = "c#d#"
// 输出：true
// 解释：s 和 t 都会变成 ""。
// 示例 3：
//
// 输入：s = "a#c", t = "b"
// 输出：false
// 解释：s 会变成 "c"，但 t 仍然是 "b"。
//
//
// 提示：
//
// 1 <= s.length, t.length <= 200
// s 和 t 只含有小写字母以及字符 '#'

// 首先想使用stack，但是要额外内存空间
// 同样扫描一次，如何处理呢
// 双指针：核心要求是要有已确定区域，但是#是往左删，是处理的过去。如果反过来，先遇到#，那么就知道下一个改如何处理
// ab#c -> ac
// c#ba -> ca reverse
// 类似于删除0保证相对顺序的题，只不过这里要多删一位
// 这里的双指针是使用skip计算
pub fn backspace_compare_review(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut slow_s:i32 = s.len() as  i32- 1;
    let mut skip_s = 0;

    let mut slow_t = t.len() as i32 - 1;
    let mut skip_t = 0;

    // 指针一起走，找到下一个有效字符，对比，
    while slow_s >= 0 || slow_t >= 0 {
        while slow_s >= 0 {
            if s[slow_s as usize] == b'#' {
                skip_s += 1;
                slow_s -= 1;
            } else if skip_s > 0{
                skip_s -= 1;
                slow_s -= 1;
            } else {
                break;
            }
        }
        while slow_t >= 0 {
            if t[slow_t as usize] == b'#' {
                skip_t += 1;
                slow_t -= 1;
            } else if skip_t > 0{
                skip_t -= 1;
                slow_t -= 1;
            } else {
                break;
            }
        }

        let s_char = if slow_s >= 0 {s[slow_s as usize]} else {b'\0'};
        let t_char = if slow_t >= 0 {t[slow_t as usize]} else {b'\0'};
        if t_char != s_char {
            return false;
        }
        slow_s -= 1;
        slow_t -= 1;
    }
    true
}

// 双指针，[0..slow]正确 (slow..fast) 待改，
pub fn backspace_compare(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut i = s.len() as i32 - 1;
    let mut j = t.len() as i32 - 1;

    let mut skip_s = 0;
    let mut skip_t = 0;

    while i >= 0 || j >= 0 {
        while i >= 0 {
            if s[i as  usize] == b'#' {
                skip_s += 1;
                i -= 1;
            } else if (skip_s > 0) {
                skip_s -= 1;
                i -= 1;
            } else {
                break;
            }
        }

        while j >= 0 {
            if t[j as usize] == b'#' {
                skip_t += 1;
                j -= 1;
            } else if (skip_t > 0) {
                skip_t -= 1;
                j -= 1;
            } else {
                break;
            }
        }

        let c1 = if i >= 0 { s[i as usize] } else { b'\0'};
        let c2 = if j >= 0 { t[j as usize] } else { b'\0'};
        if c1 != c2 {
            return false;
        }
        i -= 1;
        j -= 1;
    }
    true
}




#[test]
fn test() {
    // let s = "a#bcd##f";
    // let t = "a#bf";
    // let res = backspace_compare_review(s.to_string(), t.to_string());
    // assert!(res);
    assert!(!backspace_compare_review("aa#a".to_string(), "a".to_string()));
}