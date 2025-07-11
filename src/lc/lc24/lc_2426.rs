// https://leetcode.com/problems/number-of-pairs-satisfying-inequality/
// 2426. Number of Pairs Satisfying Inequality
pub struct Solution;
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let dnum = nums1
            .into_iter()
            .zip(nums2.into_iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();
        let mut dnums = dnum.clone();
        dnums.sort_unstable_by_key(|&k| -k);

        let mut cnt = vec![0; dnum.len() + 1];
        let inc = |cnt: &mut Vec<i32>, mut i: usize, v: i32| {
            while i < cnt.len() {
                cnt[i] += v;
                i += i & (!i + 1);
            }
        };
        let sum = |cnt: &Vec<i32>, mut i: usize| {
            let mut v = 0;
            while i > 0 {
                v += cnt[i];
                i -= i & (!i + 1);
            }
            v
        };
        for i in 1..=dnum.len() {
            inc(&mut cnt, i, 1);
        }
        let mut ans = 0;
        for d in dnum {
            let i = dnums.partition_point(|dd| *dd > d);
            inc(&mut cnt, i + 1, -1);
            let tar = d - diff;
            let j = dnums.partition_point(|dd| *dd >= tar);
            if j > 0 {
                ans += sum(&cnt, j) as i64;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_pairs() {
        assert_eq!(Solution::number_of_pairs(vec![3, 2, 5], vec![2, 2, 1], 1), 3);
        assert_eq!(Solution::number_of_pairs(vec![3, -1], vec![-2, 2], -1), 0);
    }
}
