// https://leetcode.com/problems/earliest-possible-day-of-full-bloom/
// 2136. Earliest Possible Day of Full Bloom
pub struct Solution;
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut v = plant_time
            .into_iter()
            .zip(grow_time.into_iter())
            .collect::<Vec<_>>();
        v.sort_unstable_by_key(|(_, g)| std::cmp::Reverse(*g));
        v.into_iter()
            .fold((0, 0), |(t, m), (p, g)| (t + p, m.max(t + p + g)))
            .1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_earliest_full_bloom() {
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
            9
        );
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
            9
        );
        assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
    }
}
