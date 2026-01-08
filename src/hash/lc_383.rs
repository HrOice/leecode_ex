// 给定一个赎金信 (ransom) 字符串和一个杂志(magazine)字符串，判断第一个字符串 ransom 能不能由第二个字符串 magazines 里面的字符构成。
// 如果可以构成，返回 true ；否则返回 false。
//
// (题目说明：为了不暴露赎金信字迹，要从杂志上搜索各个需要的字母，组成单词来表达意思。杂志字符串中的每个字符只能在赎金信字符串中使用一次。)
//
// 注意：
//
// 你可以假设两个字符串均只含有小写字母。
//
// canConstruct("a", "b") -> false
// canConstruct("aa", "ab") -> false
// canConstruct("aa", "aab") -> true


pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for c in magazine.chars() {
        map.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    for c in ransom_note.chars() {
        if let Some(mut count) = map.get_mut(&c) {
            *count -= 1;
            if *count < 0 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    assert!(can_construct("aa".to_string(), "aab".to_string()));
    assert!(!can_construct("aa".to_string(), "ab".to_string()));
    assert!(!can_construct("a".to_string(), "b".to_string()));

}