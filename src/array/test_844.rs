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
    let s = "a#cbd##f";
    let t = "a#bf";
    let res = backspace_compare(s.to_string(), t.to_string());
    assert!(res);
}