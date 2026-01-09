// https://leetcode.com/problems/minimum-time-to-transport-all-individuals/
// 3594. Minimum Time to Transport All Individuals
pub struct Solution;
#[derive(Debug, PartialEq)]
struct F64(f64);
impl Eq for F64 {}
impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Solution {
    pub fn min_time(n: i32, k: i32, m: i32, time: Vec<i32>, mul: Vec<f64>) -> f64 {
        if k == 1 {
            if n == 1 {
                return time[0] as f64 * mul[0];
            } else {
                return -1.0;
            }
        }
        let n = n as usize;
        let m = m as usize;
        let n2 = 1 << n;
        let mut sub = vec![vec![]; n2];
        for i in 1..n2 {
            let mut s = i;
            while s > 0 {
                while s.count_ones() as i32 > k {
                    s ^= 1 << s.trailing_zeros()
                }
                sub[i].push(s);
                s = (s - 1) & i;
            }
        }
        let mut max_t = vec![0; n2];
        for (i, t) in time.into_iter().enumerate() {
            let i2 = 1 << i;
            for j in 0..i2 {
                max_t[j | i2] = max_t[j].max(t);
            }
        }
        let mut dis = vec![vec![[f64::MAX, f64::MAX]; n2]; m];
        let mut h = std::collections::BinaryHeap::<(F64, usize, usize, usize)>::new();
        h.push((F64(0.0), 0, n2 - 1, 0));
        while let Some((F64(t), stage, mask, direction)) = h.pop() {
            if t > dis[stage][mask][direction] {
                continue;
            }
            if mask == 0 {
                return t;
            }
            if direction == 0 {
                for &submask in &sub[mask] {
                    let c = mul[stage] * max_t[submask] as f64;
                    let next_stage = (stage + c as usize) % m;
                    let next_mask = mask ^ submask;
                    if t + c < dis[next_stage][next_mask][1] {
                        dis[next_stage][next_mask][1] = t + c;
                        h.push((F64(t + c), next_stage, next_mask, 1));
                    }
                }
            } else {
                let mut r = (n2 - 1) ^ mask;
                while r > 0 {
                    let lb = 1 << r.trailing_zeros();
                    let c = mul[stage] * max_t[lb] as f64;
                    let next_stage = (stage + c as usize) % m;
                    let next_mask = mask | lb;
                    if t + c < dis[next_stage][next_mask][0] {
                        dis[next_stage][next_mask][0] = t + c;
                        h.push((F64(t + c), next_stage, next_mask, 0));
                    }
                    r ^= lb;
                }
            }
        }
        -1.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn min_time() {
        assert_approx_eq!(Solution::min_time(1, 1, 2, vec![5], vec![1.0, 1.3]), 5.0, 1e-5);
        assert_approx_eq!(
            Solution::min_time(3, 2, 3, vec![2, 5, 8], vec![1.0, 1.5, 0.75]),
            14.5,
            1e-5
        );
        assert_approx_eq!(Solution::min_time(2, 1, 2, vec![10, 10], vec![2.0, 2.0]), -1.0, 1e-5);
    }
}
