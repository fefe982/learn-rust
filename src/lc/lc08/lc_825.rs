// https://leetcode.com/problems/friends-of-appropriate-ages/
// 825. Friends Of Appropriate Ages
pub struct Solution;
impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut ages = ages;
        ages.sort_unstable();
        let mut i = ages.partition_point(|&x| x < 14);
        let mut s = i;
        let mut cnt = 0;
        let mut t;
        while i < ages.len() {
            let low = ages[i] / 2 + 7;
            let high = ages[i];
            while s < ages.len() && ages[s] <= low {
                s += 1;
            }
            t = s;
            while t < ages.len() && ages[t] <= high {
                t += 1;
            }
            if t > s {
                cnt += (t - s - 1) as i32;
            }
            i += 1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_friend_requests() {
        assert_eq!(Solution::num_friend_requests(vec![118, 14, 7, 63, 103]), 2);
        assert_eq!(Solution::num_friend_requests(vec![16, 16]), 2);
        assert_eq!(Solution::num_friend_requests(vec![16, 17, 18]), 2);
        assert_eq!(Solution::num_friend_requests(vec![20, 30, 100, 110, 120]), 3);
    }
}
