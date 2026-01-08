// 给你一个字符串数组 words ，请你找出所有在 words 的每个字符串中都出现的共用字符（ 包括重复字符），并以数组形式返回。你可以按 任意顺序 返回答案。
//
// 示例 1：
//
// 输入：words = ["bella","label","roller"] 输出：["e","l","l"]
//
// 示例 2：
//
// 输入：words = ["cool","lock","cook"] 输出：["c","o"]
//
// 提示：
//
// 1 <= words.length <= 100 1 <= words[i].length <= 100 words[i] 由小写英文字母组成

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut min_hash = vec![i32::MAX; 26];
    let mut tmp =  vec![0; 26];
    for i in 0..words.len() {
        let chars= words[i].as_bytes();
        for char in chars {
            let index = char - b'a' as u8;
            tmp[index as usize] += 1;
            //
        }
        for index in 0..26 {
            if min_hash[index] | tmp[index] > 0 {
                min_hash[index] = min_hash[index].min(tmp[index]);
            } else {
                min_hash[index] = 0;
            }
        }
        tmp = vec![0; 26];
    }

    let mut res = Vec::new();

    for (i,&r) in min_hash.iter().enumerate() {
        if r > 0 && r < i32::MAX{
            for _ in 0..r {
                res.push(String::from_utf8(vec![b'a' + i as u8]).unwrap());
            }
        }
    }
    res
}

#[test]
fn test_common_chars() {
    let words = vec!["cool","lock","cook"].iter().map(|s| s.to_string()).collect();
    let res = common_chars(words);
    println!("{:?}", res);
}