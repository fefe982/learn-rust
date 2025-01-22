// https://leetcode.com/problems/map-of-highest-peak/
// 1765. Map of Highest Peak
pub struct Solution;
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is_water = is_water;
        let mut q = std::collections::VecDeque::new();
        for i in 0..is_water.len() {
            for j in 0..is_water[i].len() {
                if is_water[i][j] == 1 {
                    q.push_back((i, j, 0));
                    is_water[i][j] = 0;
                } else {
                    is_water[i][j] = -1;
                }
            }
        }
        while let Some((i, j, l)) = q.pop_front() {
            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ni, nj) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                if ni < is_water.len() && nj < is_water[0].len() && is_water[ni][nj] == -1 {
                    is_water[ni][nj] = l + 1;
                    q.push_back((ni, nj, l + 1));
                }
            }
        }
        is_water
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(is_water: Vec<Vec<i32>>, expected: i32) {
        let actual = Solution::highest_peak(is_water.clone());
        let mut max = 0;
        assert_eq!(is_water.len(), actual.len());
        for i in 0..actual.len() {
            assert_eq!(is_water[i].len(), actual[i].len());
            for j in 0..actual[i].len() {
                max = max.max(actual[i][j]);
                if is_water[i][j] == 1 {
                    assert_eq!(actual[i][j], 0);
                }
                if i + 1 < actual.len() {
                    assert!((actual[i][j] - actual[i + 1][j]).abs() < 2);
                }
                if j + 1 < actual[i].len() {
                    assert!((actual[i][j] - actual[i][j + 1]).abs() < 2)
                }
            }
        }
        assert_eq!(max, expected);
    }
    #[test]
    fn highest_peak() {
        check(vec_vec![[0, 1], [0, 0]], 2);
        check(vec_vec![[0, 0, 1], [1, 0, 0], [0, 0, 0]], 2);
    }
}
