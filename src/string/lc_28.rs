// 28. 找出字符串中第一个匹配项的下标
// 简单
// 给你两个字符串 haystack 和 needle ，
// 请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。
// 如果 needle 不是 haystack 的一部分，则返回  -1 。
//
//
//
// 示例 1：
//
// 输入：haystack = "sadbutsad", needle = "sad"
// 输出：0
// 解释："sad" 在下标 0 和 6 处匹配。
// 第一个匹配项的下标是 0 ，所以返回 0 。
// 示例 2：
//
// 输入：haystack = "leetcode", needle = "leeto"
// 输出：-1
// 解释："leeto" 没有在 "leetcode" 中出现，所以返回 -1 。

// leeto 前缀表 0 0 0 0 0
// pattern = aabaaf, str = aabaabaaf
// 前缀表 KPM思路: 找到pattern中的重复像，减少重复匹配
// 比如aabaa 可以匹配str前5个，第6个匹配失败，
// aabaabaaf
// aabaaf
// 如果是暴力的话，就向前推进pattern
//  aabaaf
//   aabaaf
//    aabaaf
// 不断重复遍历。
// 但是pattern满足一些要求，可以看到aa有两个，那么就可以按照aa来直接移动到对应位置
// aabaabaaf
// aabaaf
//    aabaaf
// 像这样，前缀移动到pattern后续和前缀匹配的位置上 0 -> 3, 减少了前面范围的重复遍历
// 那么就需要计算一下移动的长度
// aababaaf
// aabaaf    [0,3]中匹配成功，下一个位置就是3，因为pattern[0] = pattern[3],也就是移动了3位，
//    aabaaf  移动后，第一位一定相同，只需要从第二位匹配

// aabaabaaf
// aabaaf   [0,4]匹配成功
//    aabaaf 移动3位，前两位一定相同，从第三位开始匹配

// 归纳：先移动x位，再从y开始匹配，y就是前后缀相同的长度，x = y
// 前缀表,标记的就是y
// a a b a a f
// 0 1 0 1 2 0

// a b a a b a
// 0 0 1 1 2 3

// a b b a a b b a f
// 0 0 0 1 1 2 3 4    此时j=4，a != f , j 退 = next[3] = 1

//  a  b  a  b  a  c  a  b  a  b  a  d
// [0, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 0]

fn get_next(pattern: &Vec<char>) -> Vec<usize> {
    let mut next = vec![0; pattern.len()];
    let mut j = 0;
    for i in 1..pattern.len() {
        while j > 0 && pattern[i] != pattern[j] {
            j = next[j - 1];
        }
        if pattern[i] == pattern[j] {
            j += 1;
        }
        next[i] = j;
    }
    next
}

#[test]
fn test_next() {
    assert_eq!(vec![0,1,0,1,2,0], get_next(&vec!['a','a','b','a','a', 'f']));
    assert_eq!(vec![0,0,0,1], get_next(&vec!['a','b','c','a']));
    assert_eq!(vec![0,0,1,1,2,3], get_next(&vec!['a','b','a','a','b','a']));
    let res = get_next(&"ababacababad".to_string().chars().collect::<Vec<char>>());
    println!("{:?}", res);
}


// 输入：haystack = "sadbutsad", needle = "sad"
// 输出：0

// aabaabaaf
// aabaaf
// 3
pub fn str_str(haystack: String, needle: String) -> i32 {
    let str = haystack.chars().collect::<Vec<char>>();
    let pattern = needle.chars().collect::<Vec<char>>();
    let next = get_next(&pattern);
    let mut j = 0;
    for i in 0..str.len() {
        while j > 0 && str[i] != pattern[j] {
            j = next[j - 1];
        }
        if pattern[j] == str[i] {
            j += 1;
        }
        if j == pattern.len() {
            return (i - j + 1) as i32;
        }
    }
    -1
}

#[test]
fn test_str_str() {
    assert_eq!(1, str_str("abab".to_string(), "bab".to_string()));
    assert_eq!(3, str_str("aabaabaaf".to_string(), "aabaaf".to_string()));
}