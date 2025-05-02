// https://leetcode.com/problems/beautiful-arrangement/
// 526. Beautiful Arrangement
pub struct Solution;
impl Solution {
    fn dfs(n: i32, pos: i32, used: &mut Vec<bool>) -> i32 {
        if pos > n {
            return 1;
        }
        let mut ans = 0;
        for i in 1..=n {
            if !used[i as usize] && (pos % i == 0 || i % pos == 0) {
                used[i as usize] = true;
                ans += Self::dfs(n, pos + 1, used);
                used[i as usize] = false;
            }
        }
        ans
    }
    pub fn count_arrangement(n: i32) -> i32 {
        let mut used = vec![false; n as usize + 1];
        Self::dfs(n, 1, &mut used)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_arrangement() {
        assert_eq!(Solution::count_arrangement(2), 2);
        assert_eq!(Solution::count_arrangement(1), 1);
    }
}
