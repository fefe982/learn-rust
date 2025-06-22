// https://leetcode.com/problems/count-of-smaller-numbers-after-self/
// 315. Count of Smaller Numbers After Self
pub struct Solution;
impl Solution {
    fn msort(nums: &Vec<i32>, from1: &[(usize, i32)], from2: &[(usize, i32)], to: &mut [(usize, i32)]) {
        let mut idx = 0;
        let mut idx1 = 0;
        let mut idx2 = 0;
        while idx1 < from1.len() && idx2 < from2.len() {
            if nums[from1[idx1].0] <= nums[from2[idx2].0] {
                to[idx] = (from1[idx1].0, from1[idx1].1 + idx2 as i32);
                idx1 += 1;
            } else {
                to[idx] = from2[idx2];
                idx2 += 1;
            }
            idx += 1;
        }
        while idx1 < from1.len() {
            to[idx] = (from1[idx1].0, from1[idx1].1 + from2.len() as i32);
            idx += 1;
            idx1 += 1;
        }
        while idx2 < from2.len() {
            to[idx] = from2[idx2];
            idx += 1;
            idx2 += 1;
        }
    }
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut a1: Vec<_> = (0..nums.len()).zip(std::iter::repeat(0)).collect();
        let mut a2 = vec![(0, 0); nums.len()];
        let mut l = 1;
        let mut res = vec![0; nums.len()];
        loop {
            if l >= nums.len() {
                break a1;
            }
            for s in 0..((nums.len() - 1) / (l * 2) + 1) {
                let ss = s * l * 2;
                let e = std::cmp::min(ss + l, nums.len());
                let ee = std::cmp::min(ss + l * 2, nums.len());
                Self::msort(&nums, &a1[ss..e], &a1[e..ee], &mut a2[ss..ee]);
            }
            l *= 2;
            if l >= nums.len() {
                break a2;
            }
            for s in 0..((nums.len() - 1) / (l * 2) + 1) {
                let ss = s * l * 2;
                let e = std::cmp::min(ss + l, nums.len());
                let ee = std::cmp::min(ss + l * 2, nums.len());
                Self::msort(&nums, &a2[ss..e], &a2[e..ee], &mut a1[ss..ee]);
            }
        }
        .into_iter()
        .for_each(|(i, c)| res[i] = c);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_smaller() {
        assert_eq!(Solution::count_smaller(vec![1, 2, 7, 8, 5]), vec![0, 0, 1, 1, 0]);
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }
}
