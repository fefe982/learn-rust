// https://leetcode.com/problems/parallel-courses-ii/
// 1494. Parallel Courses II
pub struct Solution;
impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let nn = 1 << n;
        let mut dp = vec![i32::MAX; nn];
        let mut need = vec![0; nn];
        dp[0] = 0;
        for r in relations {
            need[1 << (r[1] - 1) as usize] |= 1 << (r[0] - 1);
        }
        for i in 1..nn {
            need[i] = need[i & (i - 1)] | need[i & (!i + 1)];
            if (need[i] | i) != i {
                continue;
            }
            let valid = i ^ need[i];
            if valid.count_ones() <= k as u32 {
                dp[i] = std::cmp::min(dp[i], dp[i ^ valid] + 1);
            } else {
                let mut sub = valid;
                while sub > 0 {
                    if sub.count_ones() <= k as u32 {
                        dp[i] = std::cmp::min(dp[i], dp[i ^ sub] + 1);
                    }
                    sub = (sub - 1) & valid;
                }
            }
        }
        dp[nn - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_number_of_semesters() {
        assert_eq!(
            Solution::min_number_of_semesters(4, vec_vec![[2, 1], [3, 1], [1, 4]], 2),
            3
        );
        assert_eq!(
            Solution::min_number_of_semesters(5, vec_vec![[2, 1], [3, 1], [4, 1], [1, 5]], 2),
            4
        );
    }
}
