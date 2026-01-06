// 给你一个正整数 n ，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。
//
//
//
// 示例 1：
//
//
// 输入：n = 3
// 输出：[[1,2,3],[8,9,4],[7,6,5]]
// 示例 2：
//
// 输入：n = 1
// 输出：[[1]]
//
//
// 提示：
//
// 1 <= n <= 20

// m[n][n]


pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; n as usize]; n as usize]; // 内部是行
    let mut side = n as usize - 1; // 边长
    let mut side_n = 0; // 第几个边长
    let mut cur = 1;

    while side > 0 {
        let side_in_round = side_n % 4;
        let round = side_n / 4;
        if side_in_round == 0 { // 行固定
            let fix = side_n / 4;
            for i in round..side {
                res[fix][i] = cur;
                cur += 1;
            }
            side_n += 1;
        } else if side_in_round == 1 { // 列固定，行下
            let fix: usize = (n - round as i32 - 1) as usize;
            for i in round..side {
                res[i][fix] = cur;
                cur += 1;
            }
            side_n += 1;
        } else if side_in_round == 2 { // 行固定，列左
            let fix = (n - round as i32 - 1) as usize;
            for i in round..side {
                res[fix][n as usize - i - 1] = cur;
                cur += 1;
            }
            side_n += 1;
        } else {
            let fix = round;
            for i in round..side { // 列固定，行上
                res[n as usize - i - 1][fix] = cur;
                cur += 1;
            }
            side_n += 1;
            side -= 1;
        }
    }
    if n % 2 == 1 {
        res[n as usize / 2][n as usize/2] = n * n;
    }
    res
}

#[test]
fn test_59() {
    let matrix = generate_matrix(3);
    println!("{:?}", matrix);
}