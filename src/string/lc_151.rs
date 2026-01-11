// 给你一个字符串 s ，请你反转字符串中 单词 的顺序。
//
// 单词 是由非空格字符组成的字符串。s 中使用至少一个空格将字符串中的 单词 分隔开。
//
// 返回 单词 顺序颠倒且 单词 之间用单个空格连接的结果字符串。
//
// 注意：输入字符串 s中可能会存在前导空格、尾随空格或者单词间的多个空格。返回的结果字符串中，单词间应当仅用单个空格分隔，且不包含任何额外的空格。
//
//
//
// 示例 1：
//
// 输入：s = "the sky is blue"
// 输出："blue is sky the"
// 示例 2：
//
// 输入：s = "  hello world  "
// 输出："world hello"
// 解释：反转后的字符串中不能存在前导空格和尾随空格。
// 示例 3：
//
// 输入：s = "a good   example"
// 输出："example good a"
// 解释：如果两个单词间有多余的空格，反转后的字符串需要将单词间的空格减少到仅有一个。

mod review {
    // 先处理多余空格
    // 翻转全部
    // 单词翻转

    // [  hello  world  ]
    //
    pub fn reverse_words(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut write = 0;
        let mut read = 0; // 快慢指针
        while read < s.len() {
            while read < s.len() && s[read].is_ascii_whitespace() {
                read += 1;
            }
            while read < s.len() && !s[read].is_ascii_whitespace() {
                s.swap(read, write);
                write += 1;
                read += 1;
            }
            if read < s.len() {
                s[write] = ' ';
                write += 1;
            }
        }
        if write > 0 && s[write - 1] == ' ' {
            write -= 1;
        }
        s.truncate(write);
        let n = s.len();
        reverse(&mut s, 0, n - 1);
        let mut j = 0;
        for i in 0..=n {
            if i == n || s[i].is_ascii_whitespace() {
                reverse(&mut s, j, i - 1);
                j = i + 1;
            }
        }
        String::from_iter(s)
    }

    #[test]
    fn test() {
        let res = reverse_words("  hello  world  ".into());
        println!("{}", res);
        let res = reverse_words("hello  world".into());
        println!("{}", res);
    }

    fn reverse(s: &mut Vec<char>, mut start: usize, mut end: usize) {
        while start < end {
            s.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}
// split 需要额外空间，相当于O(n)
// 使用空间O(1)解决，不引入额外结构。
// 翻转整个字符串
// 翻转单词
// 去掉多余空格

use std::time::Instant;

pub fn reverse_words1(s: String) -> String {
    let mut s = s.chars().collect::<Vec<char>>();
    let len = s.len();
    //   hello  world  ..
    let last = reverse_all(&mut s); // O(n/2 + n)
    //   dlrow  olleh  ..
    // [0..slow) 确定 [slow..fast]可替换，fast

    let mut slow = 0;
    let mut fast = 0;

    while fast <= last { // O(n/2 * (n/2 + n))
        // fast 前进条件：上一个是字母，当前是空格，停止，进行处理
        if fast > 0 && s[fast] == ' ' && s[fast - 1] != ' ' {
            slow = reverse(&mut s, slow, fast - 1) + 1; // O(n/2 + n)
        } else if fast == last  {
            slow = reverse(&mut s, slow, fast); // O(n/2 + n)
        }
        fast += 1;
    }
    while s[fast - 1] == ' ' {
        fast -= 1;
    }

    String::from_iter(s[0..fast].iter())
}

// 带开头空格的翻转，翻转完返回单词结束为止
fn reverse(s: &mut Vec<char>, mut start: usize, mut end: usize) -> usize {
    let mut w_end = end;
    while start < end {
        let c = s[start];
        s[start] = s[end];
        s[end] = c;
        if c == ' ' {
            w_end -= 1;
        }
        start += 1;
        end -= 1;
    }
    while s[w_end] == ' ' {
        w_end -= 1;
    }
    w_end + 1
}

// 返回最后一个字母结尾 [  hello world] -> [dlrow olleh  ]
// [  aaaa] -> [aaaa  ] -> 3 6 -2 -1
// [aaa] -> [aaa] -> 2  3 -0 -1
fn reverse_all(s: &mut Vec<char>) -> usize {
    let mut start = 0;
    let mut end = s.len() - 1;
    let mut first_alpha_pos = 0 ; // 翻转前的第一个字母前的空格记录
    let mut first_alpha = ' ';
    while start < end {
        let c = s[start];
        if first_alpha == ' ' && c != ' ' {
            first_alpha_pos = start;
            first_alpha = c;
        }
        s[start] = s[end];
        s[end] = c;

        start += 1;
        end -= 1;
    }
    s.len() - first_alpha_pos - 1
}

// 实验
// 拆分：先压缩空格，再全部翻转，再翻转单词
pub fn reverse_words(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    if n == 0 {
        return String::new();
    }

    // 1️⃣ 压缩空格
    let mut write = 0;
    let mut read = 0;

    while read < n {
        // 跳过空格
        while read < n && chars[read] == ' ' {
            read += 1;
        }

        // 拷贝单词
        while read < n && chars[read] != ' ' {
            chars[write] = chars[read];
            write += 1;
            read += 1;
        }

        // 单词后加一个空格
        if read < n {
            chars[write] = ' ';
            write += 1;
        }
    }

    // 去掉末尾多余空格
    if write > 0 && chars[write - 1] == ' ' {
        write -= 1;
    }

    chars.truncate(write);
    let n = chars.len();

    // 2️⃣ 整体反转
    reverse_range(&mut chars, 0, n - 1);

    // 3️⃣ 逐词反转
    let mut start = 0;
    for i in 0..=chars.len() {
        if i == chars.len() || chars[i] == ' ' {
            reverse_range(&mut chars, start, i - 1);
            start = i + 1;
        }
    }

    chars.into_iter().collect()
}

fn reverse_range(chars: &mut Vec<char>, mut l: usize, mut r: usize) {
    while l < r {
        chars.swap(l, r);
        l += 1;
        r -= 1;
    }
}

#[test]
fn test_reverse() {
    let res = reverse(&mut vec![' ', ' ', ' ', 'd'], 0 as usize, 3 as usize);
    println!("{}", res);
}
#[test]
fn test_reverse_all() {
    assert_eq!(4, reverse_all(&mut vec!['a', 'b', 'c', 'd', ' ']));
    assert_eq!(3, reverse_all(&mut vec![' ', 'b', 'c', 'd', ' ']));
    assert_eq!(4, reverse_all(&mut vec!['c', 'b', 'c', 'd', 'c']));
}
#[test]
fn test_strings() {
    let t1 = Instant::now();
    // let s = "the sky is blue".to_string();
    // let res = reverse_words(s);
    // println!("{}", res);
    //
    // let s = "  hello  world  ".to_string();
    // let res = reverse_words(s);
    // println!("{}", res);
    //
    // // a good   example
    // let s = "a good   example".to_string();
    // let res = reverse_words(s);
    // println!("{}", res);
    //
    // //   Bob    Loves  Alice
    // let s = "  Bob    Loves  Alice   ".to_string();
    // let res = reverse_words(s);
    // println!("{}", res);
    //F R  I   E    N     D      S
    let s = "F R  I   E    N     D      S      ".to_string();
    let res = reverse_words(s);
    println!("{}", res);
    let t2 = Instant::now();
    println!("{:?}", t2 - t1);
}

#[test]
fn test() {
    let s = "  abcdefg";
    let mut chars = s.chars().collect::<Vec<char>>();
    let len = chars.len();
    let w_end = reverse(&mut chars, 0, len - 1);
    println!("{:?}", chars);
    println!("{:?}", w_end);
}