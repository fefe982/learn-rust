// https://leetcode.cn/problems/XxZZjK/
// LCP 70. 沙地治理
pub struct Solution;
impl Solution {
    pub fn sandy_land_management(size: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        for i in 2..=size {
            if (size - i) % 4 == 0 {
                for j in 1..=i {
                    ans.push(vec![i, j * 2 - 1]);
                }
            } else if (size - i) % 4 == 1 {
                ans.push(vec![i, 2]);
            } else if (size - i) % 4 == 2 {
                for j in 2..=i {
                    ans.push(vec![i, j * 2 - 1]);
                }
            } else {
                ans.push(vec![i, 1]);
            }
        }
        ans.push(vec![1, 1]);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(size: i32, expect: Vec<Vec<i32>>) {
        let n = expect.len();
        let ans = Solution::sandy_land_management(size);
        println!("{size}, {expect:?}, {ans:?}");
        assert_eq!(ans.len(), n);
        let mut g = vec![];
        let size = size as usize;
        for i in 0..size {
            g.push(vec![false; i * 2 + 1]);
        }
        for ig in ans {
            assert!(ig[0] >= 1 && ig[0] <= size as i32);
            assert!(ig[0] >= 1 && ig[1] <= ig[0] * 2 - 1);
            g[ig[0] as usize - 1][ig[1] as usize - 1] = true;
        }
        let mut flag = true;
        let mut grass = true;
        while flag {
            flag = false;
            grass = true;
            for i in 0..size {
                for j in 0..i * 2 + 1 {
                    if g[i][j] {
                        continue;
                    }
                    grass = false;
                    let mut ng = 0;
                    if j % 2 == 0 && i + 1 < size {
                        if g[i + 1][j + 1] {
                            ng += 1;
                        }
                    } else if j % 2 == 1 && i > 0 {
                        if g[i - 1][j - 1] {
                            ng += 1;
                        }
                    }
                    if j > 0 && g[i][j - 1] {
                        ng += 1;
                    }
                    if j < i * 2 && g[i][j + 1] {
                        ng += 1;
                    }
                    if ng >= 2 {
                        flag = true;
                        g[i][j] = true;
                    }
                }
            }
        }
        assert!(grass);
    }
    #[test]
    fn test() {
        check(
            5,
            vec_vec![
                [1, 1],
                [2, 1],
                [3, 3],
                [3, 5],
                [4, 2],
                [5, 1],
                [5, 3],
                [5, 5],
                [5, 7],
                [5, 9]
            ],
        );
        check(3, vec_vec![[1, 1], [2, 1], [2, 3], [3, 1], [3, 5]]);
        check(2, vec_vec![[1, 1], [2, 1], [2, 3]]);
    }
}
