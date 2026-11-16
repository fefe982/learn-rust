// https://leetcode.com/problems/stable-subarrays-with-equal-boundary-and-interior-sum/
// 3728. Stable Subarrays With Equal Boundary and Interior Sum
pub struct Solution;
impl Solution {
    pub fn count_stable_subarrays(capacity: Vec<i32>) -> i64 {
        let mut m: std::collections::HashMap<i32, std::collections::HashMap<i64, i32>> =
            std::collections::HashMap::new();
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..capacity.len() {
            if m.contains_key(&capacity[i]) && m[&capacity[i]].contains_key(&(sum - capacity[i] as i64)) {
                ans += m[&capacity[i]][&(sum - capacity[i] as i64)] as i64;
            }
            sum += capacity[i] as i64;
            *m.entry(capacity[i]).or_default().entry(sum).or_default() += 1;
            if i > 0 && capacity[i] == 0 && capacity[i - 1] == 0 {
                ans -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_stable_subarrays() {
        assert_eq!(Solution::count_stable_subarrays(vec![9, 3, 3, 3, 9]), 2);
        assert_eq!(Solution::count_stable_subarrays(vec![1, 2, 3, 4, 5]), 0);
        assert_eq!(Solution::count_stable_subarrays(vec![-4, 4, 0, 0, -8, -4]), 1);
    }
}
