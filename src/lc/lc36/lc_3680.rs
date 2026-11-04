// https://leetcode.com/problems/generate-schedule/
// 3680. Generate Schedule
pub struct Solution;
impl Solution {
    pub fn generate_schedule(n: i32) -> Vec<Vec<i32>> {
        if n < 5 {
            return vec![];
        }
        let mut ans = vec![];
        if n % 2 == 1 {
            for i in 0..n {
                ans.push(vec![i * 2 % n, (i * 2 + 1) % n]);
            }
        } else {
            for i in (1..n).step_by(2) {
                ans.push(vec![(i + 4) % n, (i + 5) % n]);
            }
            for i in (0..n).step_by(2) {
                ans.push(vec![i, (i + 1) % n]);
            }
        }
        for i in 2..n - 1 {
            for j in 0..n {
                ans.push(vec![j, (j + i) % n]);
            }
        }
        if n % 2 == 1 {
            for i in 0..n {
                ans.push(vec![(i * 2 + 1) % n, (i * 2) % n]);
            }
        } else {
            for i in (1..n).step_by(2) {
                ans.push(vec![i, (i + n - 1) % n]);
            }
            for i in (0..n).step_by(2) {
                ans.push(vec![(i + 2) % n, (i + 1) % n]);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: i32) {
        let v = Solution::generate_schedule(n);
        if n < 5 {
            assert!(v.is_empty());
            return;
        }
        assert_eq!(v.len(), n as usize * (n as usize - 1));
        let mut set = std::collections::HashSet::new();
        for i in 0..v.len() {
            if i > 0 {
                assert!(v[i][0] != v[i - 1][0]);
                assert!(v[i][1] != v[i - 1][1]);
                assert!(v[i][0] != v[i - 1][1]);
                assert!(v[i][1] != v[i - 1][0]);
            }
            assert!(v[i][0] != v[i][1]);
            assert!(v[i][0] >= 0 && v[i][0] < n);
            assert!(v[i][1] >= 0 && v[i][1] < n);
            assert!(set.insert((v[i][0], v[i][1])));
        }
    }
    #[test]
    fn generate_schedule() {
        check(6);
        check(3);
        check(5);
    }
}
