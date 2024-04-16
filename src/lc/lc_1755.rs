// https://leetcode.com/problems/closest-subsequence-sum/
// 1755. Closest Subsequence Sum
pub struct Solution;
impl Solution {
    fn merge(v: Vec<i32>, d: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(v.len() * 2);
        let mut i = 0;
        let mut j = 0;
        while i < v.len() && j < v.len() {
            match v[i].cmp(&(v[j] + d)) {
                std::cmp::Ordering::Less => {
                    res.push(v[i]);
                    i += 1;
                }
                std::cmp::Ordering::Equal => {
                    res.push(v[i]);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Greater => {
                    res.push(v[j] + d);
                    j += 1;
                }
            }
        }
        while i < v.len() {
            res.push(v[i]);
            i += 1;
        }
        while j < v.len() {
            res.push(v[j] + d);
            j += 1;
        }
        res
    }
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let mut vleft = vec![0];
        let mut vright = vec![0];
        for i in 0..nums.len() / 2 {
            vleft = Self::merge(vleft, nums[i]);
        }
        for i in nums.len() / 2..nums.len() {
            vright = Self::merge(vright, nums[i]);
        }
        if vleft.len() > vright.len() {
            let v = vleft;
            vleft = vright;
            vright = v;
        }
        let mut min = i32::MAX;
        let mut il = 0;
        let mut ir = vright.partition_point(|&x| x < (goal - vleft[il]));
        if ir < vright.len() {
            min = min.min(vright[ir] + vleft[il] - goal);
        }
        if ir > 0 {
            min = min.min(goal - vright[ir - 1] - vleft[il]);
        }
        il += 1;
        while il < vleft.len() {
            while ir > 0 && vright[ir - 1] + vleft[il] >= goal {
                ir -= 1;
            }
            if ir < vright.len() {
                min = min.min(vright[ir] + vleft[il] - goal);
            }
            if ir > 0 {
                min = min.min(goal - vright[ir - 1] - vleft[il]);
            }
            if ir == 0 {
                break;
            }
            il += 1;
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_abs_difference() {
        assert_eq!(Solution::min_abs_difference(vec![5, -7, 3, 5], 6), 0);
        assert_eq!(Solution::min_abs_difference(vec![7, -9, 15, -2], -5), 1);
        assert_eq!(Solution::min_abs_difference(vec![1, 2, 3], -7), 7);
    }
}
