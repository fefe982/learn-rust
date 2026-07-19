// https://leetcode.com/problems/minimum-cost-to-convert-string-iii/
// 3995. Minimum Cost to Convert String
pub struct Solution;
impl Solution {
    fn dfs(
        source: &[u8],
        target: &[u8],
        rules: &Vec<Vec<String>>,
        costs: &Vec<i32>,
        n: usize,
        dp: &mut Vec<i32>,
    ) -> i32 {
        if dp[n] >= -1 {
            return dp[n];
        }
        if n == 0 {
            return 0;
        }
        let mut ans = -1;
        if source[n - 1] == target[n - 1] {
            ans = Self::dfs(source, target, rules, costs, n - 1, dp);
        }
        'r: for i in 0..rules.len() {
            let rule = &rules[i];
            let s = rule[0].as_bytes();
            let t = rule[1].as_bytes();
            if n < t.len() {
                continue;
            }
            if &target[n - t.len()..n] != t {
                continue;
            }
            let mut c = costs[i];
            for j in 0..s.len() {
                if source[n - s.len() + j] != s[j] && s[j] != b'*' {
                    continue 'r;
                }
                if s[j] == b'*' {
                    c += 1;
                }
            }
            let last = Self::dfs(source, target, rules, costs, n - t.len(), dp);
            if last >= 0 {
                if ans < 0 {
                    ans = last + c;
                } else {
                    ans = ans.min(last + c);
                }
            }
        }
        dp[n] = ans;
        ans
    }
    pub fn min_cost(source: String, target: String, rules: Vec<Vec<String>>, costs: Vec<i32>) -> i32 {
        let source = source.as_bytes();
        let target = target.as_bytes();
        let n = source.len();
        let mut dp = vec![-2; n + 1];
        Self::dfs(source, target, &rules, &costs, n, &mut dp)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(
            Solution::min_cost(
                "hello".to_string(),
                "world".to_string(),
                vec_vec_str![["he", "wo"], ["llo", "rld"]],
                vec![3, 4]
            ),
            7
        );
        assert_eq!(
            Solution::min_cost(
                "cat".to_string(),
                "dog".to_string(),
                vec_vec_str![["c*t", "dog"]],
                vec![2]
            ),
            3
        );
        assert_eq!(
            Solution::min_cost(
                "test".to_string(),
                "next".to_string(),
                vec_vec_str![["*e*t", "next"]],
                vec![4]
            ),
            6
        );
        assert_eq!(
            Solution::min_cost("ab".to_string(), "bc".to_string(), vec_vec_str![["a*", "bd"]], vec![9]),
            -1
        );
    }
}
