// 题目描述
// 字符串的右旋转操作是把字符串尾部的若干个字符转移到字符串的前面。
// 给定一个字符串 s 和一个正整数 k，请编写一个函数，将字符串中的后面 k 个字符移到字符串的前面，实现字符串的右旋转操作。
//
// 例如，对于输入字符串 "abcdefg" 和整数 2，函数应该将其转换为 "fgabcde"。
//
// 输入描述
// 输入共包含两行，第一行为一个正整数 k，代表右旋转的位数。第二行为字符串 s，代表需要旋转的字符串。
// 输出描述
// 输出共一行，为进行了右旋转操作后的字符串。
// 输入示例
// 2
// abcdefg
// 输出示例
// fgabcde
// 提示信息
// 数据范围：
// 1 <= k < 10000,
// 1 <= s.length < 10000;

mod review {
    // 翻转单词
    fn kama_055(s: String, k: usize) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        reverse(&mut s, 0, n - 1);
        reverse(&mut s, k, n - 1);
        reverse(&mut s, 0, k - 1);
        String::from_iter(s.into_iter())
    }

    fn reverse(s: &mut Vec<char>, mut start: usize, mut end: usize) {
        while start < end {
            s.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
    #[test]
    fn test() {
        assert_eq!(crate::string::kama_055::kama_055("abcdefg".to_string(), 1), "gabcdef");
        assert_eq!(crate::string::kama_055::kama_055("abcdefg".to_string(), 2), "fgabcde");
        assert_eq!(crate::string::kama_055::kama_055("abcdefg".to_string(), 3), "efgabcd");
        assert_eq!(crate::string::kama_055::kama_055("abcdefg".to_string(), 4), "defgabc");
        assert_eq!(crate::string::kama_055::kama_055("abcdefg".to_string(), 5), "cdefgab");
        assert_eq!(crate::string::kama_055::kama_055("abcdefg".to_string(), 6), "bcdefga");
        assert_eq!(crate::string::kama_055::kama_055("abcdefg".to_string(), 7), "abcdefg");
        assert_eq!(crate::string::kama_055::kama_055("abcdef".to_string(), 2), "efabcd");
    }
}
// abcdefg -> fgcdeab -> fgabecd
// 双指针，right指向倒数k，left从0开始，交换，前进
// 这个思路在 s >= 2*k时成立，其他情况不成立，所以不可用
fn kama_055_1(s: String, k: usize) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = chars.len() - k;
    while left < right {
        while right < s.len() {
            chars.swap(left, right);
            left += 1;
            right += 1;
        }
        right = chars.len() - k;
    }
    String::from_iter(chars)
}

// 这个题和lc151类似，反转单词，但是分隔符不是空格而是位置，相对简单
fn kama_055(s: String, k: usize) -> String {
    let n = s.len();
    let mut s = s.chars().collect::<Vec<char>>();
    reverse(&mut s, 0, n - 1);
    reverse(&mut s, 0, k-1);
    reverse(&mut s, k, n-1);
    String::from_iter(s)
}

fn reverse(s: &mut Vec<char>, mut start: usize, mut end: usize) {
    while start < end {
        s.swap(start, end);
        start += 1;
        end -= 1;
    }
}

#[test]
fn test() {
    assert_eq!(kama_055("abcdefg".to_string(), 1), "gabcdef");
    assert_eq!(kama_055("abcdefg".to_string(), 2), "fgabcde");
    assert_eq!(kama_055("abcdefg".to_string(), 3), "efgabcd");
    assert_eq!(kama_055("abcdefg".to_string(), 4), "defgabc");
    assert_eq!(kama_055("abcdefg".to_string(), 5), "cdefgab");
    assert_eq!(kama_055("abcdefg".to_string(), 6), "bcdefga");
    assert_eq!(kama_055("abcdefg".to_string(), 7), "abcdefg");
    assert_eq!(kama_055("abcdef".to_string(), 2), "efabcd");
}