// https://leetcode.com/problems/maximize-the-minimum-powered-city/
// 2528. Maximize the Minimum Powered City
pub struct Solution;
impl Solution {
    fn check(power: &Vec<i64>, r: usize, k: i32, m: i64) -> bool {
        if r == 0 {
            let mut c = 0;
            for &p in power {
                c += (m - p).max(0);
                if c > k as i64 {
                    return false;
                }
            }
            return true;
        }
        let mut q = std::collections::VecDeque::from_iter(std::iter::repeat(0).take(r * 2));
        let mut add = 0;
        let mut c = 0;
        for &p in power {
            let np = p + add;
            let nadd = (m - np).max(0);
            c += nadd;
            if c > k as i64 {
                return false;
            }
            q.push_back(nadd);
            add += nadd;
            add -= q.pop_front().unwrap_or(0);
        }
        true
    }
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let mut s = stations.iter().take(r as usize).fold(0, |s, &x| x as i64 + s);
        let mut power = Vec::with_capacity(stations.len());
        let r = r as usize;
        let mut low = i64::MAX;
        for i in 0..stations.len() {
            if i + r < stations.len() {
                s += stations[i + r] as i64;
            }
            if i >= r + 1 {
                s -= stations[i - r - 1] as i64;
            }
            low = low.min(s);
            power.push(s);
        }
        if k == 0 {
            return low;
        }
        let mut high = low + k as i64;
        if Self::check(&power, r, k, high) {
            return high;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check(&power, r, k, mid) {
                low = mid;
            } else {
                high = mid;
            }
        }
        low
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_power() {
        assert_eq!(
            Solution::max_power(vec![57, 70, 35, 30, 29, 13, 17, 88, 89, 49], 1, 90),
            138
        );
        assert_eq!(Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2), 5);
        assert_eq!(Solution::max_power(vec![4, 4, 4, 4], 0, 3), 4);
    }
}
