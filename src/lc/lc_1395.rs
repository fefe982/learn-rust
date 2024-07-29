// https://leetcode.com/problems/count-number-of-teams/
// 1395. Count Number of Teams
pub struct Solution;
impl Solution {
    fn inc(cnt: &mut Vec<i32>, n: i32, c: i32) {
        let mut n = n as usize;
        while n < cnt.len() {
            cnt[n] += c;
            n += n & (!n + 1);
        }
    }
    fn get(cnt: &Vec<i32>, n: i32) -> i32 {
        let mut n = n as usize;
        let mut ans = 0;
        while n > 0 {
            ans += cnt[n];
            n -= n & (!n + 1);
        }
        ans
    }
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut cnt1 = vec![0; 100001];
        let mut cnt2_inc = vec![0; 100001];
        let mut cnt2_dec = vec![0; 100001];
        let mut ans = 0;
        for (i, r) in rating.into_iter().enumerate() {
            ans += Self::get(&cnt2_inc, r - 1) + Self::get(&cnt2_dec, 100001 - r - 1);
            Self::inc(&mut cnt2_inc, r, Self::get(&cnt1, r - 1));
            Self::inc(&mut cnt2_dec, 100001 - r, i as i32 - Self::get(&cnt1, r));
            Self::inc(&mut cnt1, r, 1);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_teams() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
    }
}
