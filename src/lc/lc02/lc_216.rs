// https://leetcode.com/problems/combination-sum-iii/
// 216. Combination Sum III
pub struct Solution;
impl Solution {
    fn comb(start: i32, target: i32, n: i32, v: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>) {
        if target == 0 && n == 0 {
            v.push(cur.clone());
            return;
        }
        if target < 0 || n < 0 {
            return;
        }
        if target != 0 && n == 0 {
            return;
        }
        if (start * 2 + n - 1) * n / 2 > target {
            return;
        }
        if (9 * 2 - n + 1) * n / 2 < target {
            return;
        }
        if n == 1 {
            cur.push(target);
            v.push(cur.clone());
            cur.pop();
        } else {
            for i in start..=9 - n + 1 {
                cur.push(i);
                Self::comb(i + 1, target - i, n - 1, v, cur);
                cur.pop();
            }
        }
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::comb(1, n, k, &mut ans, &mut vec![]);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cmobination_sum3() {
        assert_eq!(Solution::combination_sum3(3, 7), vec_vec![[1, 2, 4]]);
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec_vec![[1, 2, 6], [1, 3, 5], [2, 3, 4]]
        );
        assert_eq!(Solution::combination_sum3(4, 1), Vec::<Vec<i32>>::new());
    }
}
