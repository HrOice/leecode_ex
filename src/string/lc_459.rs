// 给定一个非空的字符串 s ，检查是否可以通过由它的一个子串重复多次构成。
//
//
//
// 示例 1:
//
// 输入: s = "abab"
// 输出: true
// 解释: 可由子串 "ab" 重复两次构成。
// 示例 2:
//
// 输入: s = "aba"
// 输出: false
// 示例 3:
//
// 输入: s = "abcabcabcabc"
// 输出: true
// 解释: 可由子串 "abc" 重复四次构成。 (或子串 "abcabc" 重复两次构成。)

mod review {
    use crate::string::lc_459::get_next;

    // 重复字符串可以看kmp，推导过程
    // next数组表达的意义：在当前位置上最长的相同前后缀长度，这个长度也可以表示从0开始的位置
    // 如果next[cur] > 0, 那么s[m] = s[cur]
    // n为字符串长度，那么 k = next[n - 1] 也就是最后一个字符的next元素
    // s[k] = s[n - 1]
    // s[k - x] = s[n - x - 1]
    // s[x] = s[n - k + x] 表示 s[0..x] = s[n - k + x .. n - 1]
    // p = n - k 表示 ababab 001234, k = 4 -> abab|ab| 尾部区间
    // [0..p-1][p..2p-1][2p..3p-1]... 分块，块长p，也就是重复字符的最小长度
    // n = p x m -> n % p == 0
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s= s.chars().collect::<Vec<char>>();
        let next = get_next(&s); 
        let k = next[s.len() - 1];
        if k == 0 {
            return false;
        }
        let n = s.len();
        let p = n - k;
        n % p == 0
    }
}

// KMP
// a b c a b c a b c a b c
// 0 0 0 1 2 3 4 5 6 7 8 9
// k = next[n - 1] 在n - 1处的最大相等前后缀长度
// s[0..k-1] = s[n-k..n-1]
// 对所有 0 ≤ x < k：
// s[x] == s[n - k + x] n-k表示起点
// s[p1 + x] = s[p2 + x] p = n-k
// p ≤ i < n 也就是右侧s[n-k..n-1]区间中，再往前一个区间的相对位置相同
// s[i] = s[i - p]
// s = [0..p-1][p..2p-1][2p..3p-1]...
// n是长度 n = p x m
// n % p == 0
pub fn repeated_substring_pattern(s: String) -> bool {
    let s= s.chars().collect::<Vec<char>>();
    let next = get_next(&s);
    let n = s.len();
    let k = next[n - 1];
    if k == 0 {
        return false
    }
    let p = n - k;
    n % p == 0
}

#[test]
fn test() {
    // assert!(!repeated_substring_pattern("abac".to_string()));
    assert!(repeated_substring_pattern("abab".to_string()));
    // ababab
    // 001234
    assert!(repeated_substring_pattern("ababab".to_string()));

}

//  aabaaf
fn get_next(s: &Vec<char>) -> Vec<usize>{
    let mut res = vec![0; s.len()];
    let mut j = 0;
    for i in 1..s.len() {
        while j > 0 && s[i] != s[j] {
            j = res[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        res[i] = j;
    }
    res
}

#[test]
fn test_next() {
    assert_eq!(vec![0,1,0,1,2,0], get_next(&vec!['a','a','b','a','a', 'f']));
    assert_eq!(vec![0,0,0,1], get_next(&vec!['a','b','c','a']));
    assert_eq!(vec![0,0,1,1,2,3], get_next(&vec!['a','b','a','a','b','a']));
    let res = get_next(&"abcabcabcabc".to_string().chars().collect::<Vec<char>>());
    println!("{:?}", res);
}



pub fn repeated_substring_pattern_1(s: String) -> bool {
    let mut new_s = s.clone();
    new_s.push_str(s.clone().as_str());
    let ss = &(&new_s.as_str())[1..s.len() * 2 - 1];
    ss.contains(s.as_str())
}

#[test]
fn test_repeated_substring_pattern() {
    assert!(repeated_substring_pattern("abc".to_string()));
}