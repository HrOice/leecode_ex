// 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的 字母异位词。
//
//
//
// 示例 1:
//
// 输入: s = "anagram", t = "nagaram"
// 输出: true
// 示例 2:
//
// 输入: s = "rat", t = "car"
// 输出: false
//
//
// 提示:
//
// 1 <= s.length, t.length <= 5 * 104
// s 和 t 仅包含小写字母

pub fn is_anagram(s: String, t: String) -> bool {
    let mut v = vec![0; 26];
    let s = s.as_bytes();
    let t = t.as_bytes();
    for c in s {
        v[(c - b'a') as usize] += 1;
    }
    for c in t {
        v[(c - b'a') as usize] -= 1;
    }
    for i in v {
        if i != 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_lc_242() {
    assert!(!is_anagram("ab".to_string(), "a".to_string()));
}