// 给定一个字符串 s，它包含小写字母和数字字符，请编写一个函数，将字符串中的字母字符保持不变，而将每个数字字符替换为number。
//
// 例如，对于输入字符串 "a1b2c3"，函数应该将其转换为 "anumberbnumbercnumber"。
//
// 对于输入字符串 "a5b"，函数应该将其转换为 "anumberb"
//
// 输入：一个字符串 s,s 仅包含小写字母和数字字符。
//
// 输出：打印一个新的字符串，其中每个数字字符都被替换为了number
//
// 样例输入：a1b2c3
//
// 样例输出：anumberbnumbercnumber
//
// 数据范围：1 <= s.length < 10000。


// a1b2c3--------
fn replace_by_number(s: String) -> String {
    let mut chars = s.chars()
        .collect::<Vec<char>>();
    let len = chars.len();
    let num_count = chars.iter().filter(|c| **c >= '0' && **c <= '9').count();
    chars.resize(s.len() + num_count * 5, '_');
    let mut cur = len - 1;
    let mut pos = chars.len() - 1;
    while cur >= 0 {
        if chars[cur] >= '0' && chars[cur] <= '9' {
            chars[pos] = 'r';
            chars[pos-1] = 'e';
            chars[pos-2] = 'b';
            chars[pos-3] = 'm';
            chars[pos-4] = 'u';
            chars[pos-5] = 'n';
            if pos > 6 {
                pos -= 6;
            }
        } else {
            chars[pos] = chars[cur];
            if pos > 0 {
                pos -= 1;
            }
        }
        if cur > 0 {
            cur -= 1;
        } else {
            break;
        }
    }
    String::from_iter(chars)
}

#[test]
fn test() {
    let res = replace_by_number(String::from("3a4"));
    println!("{}", res);
}