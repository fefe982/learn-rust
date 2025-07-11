// https://leetcode.com/problems/find-all-groups-of-farmland/
// 1992. Find All Groups of Farmland
pub struct Solution;
impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut land = land;
        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 1 {
                    let mut ii = i;
                    let mut jj = j;
                    while ii < land.len() && land[ii][jj] == 1 {
                        ii += 1;
                    }
                    while jj < land[0].len() && land[ii - 1][jj] == 1 {
                        jj += 1;
                    }
                    for l in i..ii {
                        for k in j..jj {
                            land[l][k] = 0;
                        }
                    }
                    ans.push(vec![i as i32, j as i32, (ii - 1) as i32, (jj - 1) as i32]);
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_farmland() {
        assert_eq!(
            Solution::find_farmland(vec_vec![[1, 0, 0], [0, 1, 1], [0, 1, 1]]),
            vec_vec![[0, 0, 0, 0], [1, 1, 2, 2]]
        );
        assert_eq!(
            Solution::find_farmland(vec_vec![[1, 1], [1, 1]]),
            vec_vec![[0, 0, 1, 1]]
        );
        assert_eq!(Solution::find_farmland(vec_vec![[0]]), Vec::<Vec<i32>>::new());
    }
}
