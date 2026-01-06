// https://leetcode.com/problems/count-routes-to-climb-a-rectangular-grid/
// 3797. Count Routes to Climb a Rectangular Grid
pub struct Solution;
impl Solution {
    pub fn number_of_routes(grid: Vec<String>, d: i32) -> i32 {
        let len = grid[0].len();
        let mut from_last = vec![0; len + 1];
        let mut cnt_all = vec![0; len + 1];
        const MOD: i64 = 1_000_000_007;
        let d = d as usize;
        for l in 0..grid.len() {
            for (i, c) in grid[l].chars().enumerate() {
                if c == '.' {
                    if l == 0 {
                        from_last[i + 1] = from_last[i] + 1;
                    } else {
                        from_last[i + 1] =
                            (from_last[i] + cnt_all[(i + d).min(len)] - cnt_all[(i + 1).saturating_sub(d)] + MOD) % MOD;
                    }
                } else {
                    from_last[i + 1] = from_last[i];
                }
            }
            for (i, c) in grid[l].chars().enumerate() {
                if c == '.' {
                    cnt_all[i + 1] =
                        (cnt_all[i] + from_last[(i + d + 1).min(len)] - from_last[i.saturating_sub(d)] + MOD) % MOD;
                } else {
                    cnt_all[i + 1] = cnt_all[i];
                }
            }
        }
        cnt_all[len] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn number_of_routes() {
        assert_eq!(Solution::number_of_routes(vec_str!["..", "#."], 1), 2);
        assert_eq!(Solution::number_of_routes(vec_str!["..", "#."], 2), 4);
        assert_eq!(Solution::number_of_routes(vec_str!["#"], 750), 0);
        assert_eq!(Solution::number_of_routes(vec_str![".."], 1), 4);
    }
}
