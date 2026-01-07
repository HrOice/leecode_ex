// 你正在探访一家农场，农场从左到右种植了一排果树。这些树用一个整数数组 fruits 表示，其中 fruits[i] 是第 i 棵树上的水果 种类 。
//
// 你想要尽可能多地收集水果。然而，农场的主人设定了一些严格的规矩，你必须按照要求采摘水果：
//
// 你只有 两个 篮子，并且每个篮子只能装 单一类型 的水果。每个篮子能够装的水果总量没有限制。
// 你可以选择任意一棵树开始采摘，你必须从 每棵 树（包括开始采摘的树）上 恰好摘一个水果 。采摘的水果应当符合篮子中的水果类型。每采摘一次，你将会向右移动到下一棵树，并继续采摘。
// 一旦你走到某棵树前，但水果不符合篮子的水果类型，那么就必须停止采摘。
// 给你一个整数数组 fruits ，返回你可以收集的水果的 最大 数目。
//
//
//
// 示例 1：
//
// 输入：fruits = [1,2,1]
// 输出：3
// 解释：可以采摘全部 3 棵树。
// 示例 2：
//
// 输入：fruits = [0,1,2,2]
// 输出：3
// 解释：可以采摘 [1,2,2] 这三棵树。
// 如果从第一棵树开始采摘，则只能采摘 [0,1] 这两棵树。
// 示例 3：
//
// 输入：fruits = [1,2,3,2,2]
// 输出：4
// 解释：可以采摘 [2,3,2,2] 这四棵树。
// 如果从第一棵树开始采摘，则只能采摘 [1,2] 这两棵树。
// 示例 4：
//
// 输入：fruits = [3,3,3,1,2,1,1,2,3,3,4]
// 输出：5
// 解释：可以采摘 [1,2,1,1,2] 这五棵树。
//
//
// 提示：
//
// 1 <= fruits.length <= 105
// 0 <= fruits[i] < fruits.length

// 可以使用map，但是为了减少空间使用，需要其他办法
// 就是中间连续最大长度，中间的条件是不能有第三个数字
// 可以考虑指针
// [3,3,3,1,2,1,1,2,3,3,4]
//  s     t f              这个时候要把s移动到t，f继续走 。 s = 0, f = 4, t = 0,如何定位新t，用两个指针处理，这两个指针遇到新的水果更新，有些复杂
//        s       t f      这时s要移动到t，t是2的开始
//                s t   f  这时t是连续3开始
// t就是记录f之前连续数字的开始位置
// f 前进，t如何定位，
// 规律：遇到种类之内的，长度+1，否则最后连续长度 + 1
//

pub fn total_fruit_review(fruits: Vec<i32>) -> i32 {
    let mut slow = -1;
    let mut last = 0; // 上一个水果
    let mut second_last = 0; // 上上个水果
    let mut last_count = 0;
    let mut res = 0;
    let mut cur_len = 0;

    for fast in 0..fruits.len() {
        let fruit = fruits[fast];
        if fruit == last || fruit == second_last {
            cur_len += 1;
        } else {
            cur_len = 1 + last_count;
        }
        if fruit == last {
            last_count += 1;
        } else {
            second_last = last;
            last = fruit;
            last_count = 1;
        }
        res = res.max(cur_len);
    }

    res as i32

}

use std::collections::HashMap;

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut basket = HashMap::<i32, i32>::new();
    let mut j = 0;
    for (i, fruit) in fruits.iter().enumerate() {
        *basket.entry(*fruit).or_insert(0) += 1;

        while basket.len() > 2 {
            let d_fruit = fruits[j];
            if let Some(count) = basket.get_mut(&d_fruit) {
                *count -= 1;
                if *count == 0 {
                    basket.remove(&d_fruit);
                }
            }
            j += 1;
        }
        res = res.max(i + 1 - j);
    }
    res as i32
}

// [3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]
// 记录两个fruit，记录当前开头连续的数量，当遇到第三个水果时，要找到最后一段相同的水果 + 第三个
pub fn total_fruit1(fruits: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut last = -1;
    let mut s_last = -1;
    let mut last_count = 0;
    let mut cur_len = 0;

    for &fruit in fruits.iter() {
        if fruit == last || fruit == s_last {
            cur_len += 1;
        } else {
            cur_len = last_count + 1;
        }

        if fruit == last {
            last_count += 1;
        } else {
            s_last = last;
            last_count = 1;
            last = fruit;
        }
        res = res.max(cur_len);
    }
    res as i32
}

#[test]
fn test() {
    let fruits = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
    let res = total_fruit_review(fruits);
    assert_eq!(res, 5);
}
