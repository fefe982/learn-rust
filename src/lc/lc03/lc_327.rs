// https://leetcode.com/problems/count-of-range-sum/
// 327. Count of Range Sum
pub struct Solution;
impl Solution {
    fn msort(from1: &[i64], from2: &[i64], to: &mut [i64], lower: i64, upper: i64) -> i32 {
        let mut il = 0;
        let mut ih = 0;
        let mut cnt = 0;
        for i in 0..from1.len() {
            while il < from2.len() && from2[il] - from1[i] < lower {
                il += 1;
            }
            while ih < from2.len() && from2[ih] - from1[i] <= upper {
                ih += 1;
            }
            cnt += (ih - il) as i32;
        }
        let mut idx = 0;
        let mut idx1 = 0;
        let mut idx2 = 0;
        while idx1 < from1.len() && idx2 < from2.len() {
            if from1[idx1] <= from2[idx2] {
                to[idx] = from1[idx1];
                idx1 += 1;
            } else {
                to[idx] = from2[idx2];
                idx2 += 1;
            }
            idx += 1;
        }
        while idx1 < from1.len() {
            to[idx] = from1[idx1];
            idx += 1;
            idx1 += 1;
        }
        while idx2 < from2.len() {
            to[idx] = from2[idx2];
            idx += 1;
            idx2 += 1;
        }
        cnt
    }
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut cnt = 0;
        let mut a1 = [0]
            .into_iter()
            .chain(nums.into_iter())
            .scan(0i64, |s, n| {
                *s += n as i64;
                Some(*s)
            })
            .collect::<Vec<i64>>();
        let mut a2 = vec![0i64; a1.len()];
        let mut l = 1;
        loop {
            if l >= a1.len() {
                break;
            }
            for s in 0..((a1.len() - 1) / (l * 2) + 1) {
                let ss = s * l * 2;
                let e = std::cmp::min(ss + l, a1.len());
                let ee = std::cmp::min(ss + l * 2, a1.len());
                cnt += Self::msort(
                    &a1[ss..e],
                    &a1[e..ee],
                    &mut a2[ss..ee],
                    lower as i64,
                    upper as i64,
                );
            }
            l *= 2;
            if l >= a1.len() {
                break;
            }
            for s in 0..((a1.len() - 1) / (l * 2) + 1) {
                let ss = s * l * 2;
                let e = std::cmp::min(ss + l, a1.len());
                let ee = std::cmp::min(ss + l * 2, a1.len());
                cnt += Self::msort(
                    &a2[ss..e],
                    &a2[e..ee],
                    &mut a1[ss..ee],
                    lower as i64,
                    upper as i64,
                );
            }
            l *= 2;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_range_sum() {
        assert_eq!(Solution::count_range_sum(vec![0, -3, -3, 1, 1, 2], 3, 5), 2);
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
        assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
    }
}
