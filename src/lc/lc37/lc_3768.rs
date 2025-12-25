// https://leetcode.com/problems/minimum-inversion-count-in-subarrays-of-fixed-length/
// 3768. Minimum Inversion Count in Subarrays of Fixed Length
pub struct Solution;
impl Solution {
    pub fn min_inversion_count(nums: Vec<i32>, k: i32) -> i64 {
        let mut m = std::collections::BTreeMap::new();
        for &n in &nums {
            *m.entry(n).or_insert(0) += 1;
        }
        for ((_, v), i) in m.iter_mut().zip(1..) {
            *v = i;
        }
        let l = m.len();
        let nums = nums.into_iter().map(|n| *m.get(&n).unwrap()).collect::<Vec<_>>();
        let mut cnt = vec![0; l + 1];
        fn add(cnt: &mut Vec<i64>, mut i: usize, v: i64) {
            while i < cnt.len() {
                cnt[i] += v;
                i += i & (!i + 1);
            }
        }
        fn get(cnt: &Vec<i64>, mut i: usize) -> i64 {
            let mut res = 0;
            while i > 0 {
                res += cnt[i];
                i -= i & (!i + 1);
            }
            res
        }
        let mut min = i64::MAX;
        let k = k as usize;
        let mut sum = 0;
        for (i, &n) in nums.iter().enumerate() {
            add(&mut cnt, n as usize, 1);
            let inv = (i as i64 + 1).min(k as i64) - get(&cnt, n as usize);
            sum += inv;
            if i >= k - 1 {
                min = min.min(sum);
                if min == 0 {
                    break;
                }
            }
            if i >= k - 1 {
                let rm = nums[i + 1 - k];
                sum -= get(&cnt, rm as usize - 1);
                add(&mut cnt, rm as usize, -1);
            }
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_inversion_count() {
        assert_eq!(Solution::min_inversion_count(vec![3, 1, 2, 5, 4], 3), 0);
        assert_eq!(Solution::min_inversion_count(vec![5, 3, 2, 1], 4), 6);
    }
}
