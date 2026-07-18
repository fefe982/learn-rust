// https://leetcode.com/problems/number-of-black-blocks/
// 2768. Number of Black Blocks
pub struct Solution;
impl Solution {
    pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
        let mut cnt = std::collections::HashMap::new();
        for v in coordinates {
            for i in -1..=0 {
                for j in -1..=0 {
                    let (x, y) = (v[0] + i, v[1] + j);
                    if x >= 0 && x < m - 1 && y >= 0 && y < n - 1 {
                        *cnt.entry((x, y)).or_insert(0) += 1;
                    }
                }
            }
        }
        let mut ans = vec![0; 5];
        let mut c = 0;
        for v in cnt.values() {
            ans[*v as usize] += 1;
            c += 1;
        }
        ans[0] = (m as i64 - 1) * (n as i64 - 1) - c;
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_black_blocks() {
        assert_eq!(
            Solution::count_black_blocks(3, 3, vec_vec![[0, 0]]),
            vec![3, 1, 0, 0, 0]
        );
        assert_eq!(
            Solution::count_black_blocks(3, 3, vec_vec![[0, 0], [1, 1], [0, 2]]),
            vec![0, 2, 2, 0, 0]
        );
    }
}
