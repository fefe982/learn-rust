// https://leetcode.com/problems/count-artifacts-that-can-be-extracted/
// 2201. Count Artifacts That Can Be Extracted
pub struct Solution;
impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![false; n as usize]; n as usize];
        for d in dig {
            grid[d[0] as usize][d[1] as usize] = true;
        }
        let mut ans = 0;
        for a in artifacts {
            let mut can_dig = true;
            for i in a[0]..=a[2] {
                for j in a[1]..=a[3] {
                    if !grid[i as usize][j as usize] {
                        can_dig = false;
                        break;
                    }
                }
                if !can_dig {
                    break;
                }
            }
            if can_dig {
                ans += 1;
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
    fn test_dig_artifacts() {
        assert_eq!(
            Solution::dig_artifacts(2, vec_vec![[0, 0, 0, 0], [0, 1, 1, 1]], vec_vec![[0, 0], [0, 1]]),
            1
        );
        assert_eq!(
            Solution::dig_artifacts(
                2,
                vec_vec![[0, 0, 0, 0], [0, 1, 1, 1]],
                vec_vec![[0, 0], [0, 1], [1, 1]]
            ),
            2
        );
    }
}
