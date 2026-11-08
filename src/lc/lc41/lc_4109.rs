// https://leetcode.com/problems/count-valid-suffix-sorts-ii/
// 4109. Count Valid Suffix Sorts II
pub struct Solution;
impl Solution {
    fn count(nums: &Vec<i32>, target: &Vec<i32>) -> Vec<bool> {
        let mut cnt = std::collections::HashMap::new();
        for i in 0..nums.len() {
            *cnt.entry(nums[i]).or_insert(0) += 1;
        }
        let mut v = vec![false; target.len() - nums.len() + 1];
        let mut l = 0;
        let mut r = 0;
        while r < target.len() {
            let x = target[r];
            *cnt.entry(x).or_default() -= 1;
            while cnt[&x] < 0 {
                let y = target[l];
                *cnt.entry(y).or_default() += 1;
                l += 1;
            }
            if r + 1 - l == nums.len() {
                v[l] = true;
            }
            r += 1;
        }
        v
    }
    fn get_z(s: Vec<i32>) -> Vec<usize> {
        let mut z = vec![0; s.len()];
        let mut l = 0;
        let mut r = 0;
        for i in 1..s.len() {
            if i <= r {
                z[i] = z[i - l].min(r - i + 1);
            }
            while i + z[i] < s.len() && s[z[i]] == s[i + z[i]] {
                l = i;
                r = i + z[i];
                z[i] += 1;
            }
        }
        z
    }
    pub fn count_suffix(nums: Vec<i32>, target: Vec<i32>) -> Vec<i32> {
        let v = Self::count(&nums, &target);
        let z = Self::get_z(Vec::from_iter(nums.iter().chain(target.iter()).cloned()));
        let mut inc = vec![0; target.len()];
        for i in 1..target.len() {
            if target[i - 1] < target[i] {
                inc[i] = i;
            } else {
                inc[i] = inc[i - 1];
            }
        }
        let mut ans = vec![0; v.len()];
        for i in 0..v.len() {
            if v[i] {
                let pre = z[nums.len() + i].min(nums.len() - 1);
                let suf = if inc[i + nums.len() - 1] >= i {
                    inc[i + nums.len() - 1] - i
                } else {
                    0
                };
                ans[i] = if pre + 1 >= suf { (pre + 1 - suf) as i32 } else { 0 };
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_suffix() {
        assert_eq!(
            Solution::count_suffix(vec![2, 2, 3], vec![2, 2, 3, 2, 1, 3, 2, 2]),
            vec![1, 1, 0, 0, 0, 1]
        );
        assert_eq!(
            Solution::count_suffix(vec![5, 6, 5], vec![5, 6, 5, 5, 6, 5]),
            vec![2, 1, 0, 2]
        );
    }
}
