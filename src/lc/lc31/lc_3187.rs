// https://leetcode.com/problems/peaks-in-array/
// 3187. Peaks in Array
pub struct Solution;
impl Solution {
    fn add(v: &mut Vec<i32>, mut i: usize, val: i32) {
        while i < v.len() {
            v[i] += val;
            i += i & (!i + 1);
        }
    }
    fn get(v: &Vec<i32>, mut i: usize) -> i32 {
        let mut ret = 0;
        while i > 0 {
            ret += v[i];
            i -= i & (!i + 1);
        }
        ret
    }
    pub fn count_of_peaks(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = vec![0; nums.len() + 1];
        let mut nums = nums;
        for i in 1..nums.len() - 1 {
            if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
                Self::add(&mut v, i + 1, 1);
            }
        }
        let mut ans = vec![];
        for q in queries {
            if q[0] == 1 {
                if q[2] - q[1] <= 1 {
                    ans.push(0);
                } else {
                    ans.push(Self::get(&v, q[2] as usize) - Self::get(&v, q[1] as usize + 1));
                }
            } else {
                let idx = q[1] as usize;
                if idx > 1 && nums[idx - 1] > nums[idx - 2] {
                    if nums[idx] >= nums[idx - 1] && nums[idx - 1] > q[2] {
                        Self::add(&mut v, idx, 1);
                    }
                    if nums[idx] < nums[idx - 1] && nums[idx - 1] <= q[2] {
                        Self::add(&mut v, idx, -1);
                    }
                }
                if idx > 0 && idx < nums.len() - 1 {
                    let bound = nums[idx - 1].max(nums[idx + 1]);
                    if bound < nums[idx] && bound >= q[2] {
                        Self::add(&mut v, idx + 1, -1);
                    }
                    if bound >= nums[idx] && bound < q[2] {
                        Self::add(&mut v, idx + 1, 1);
                    }
                }
                if idx < nums.len() - 2 && nums[idx + 1] > nums[idx + 2] {
                    if nums[idx + 1] > nums[idx] && nums[idx + 1] <= q[2] {
                        Self::add(&mut v, idx + 2, -1);
                    }
                    if nums[idx + 1] <= nums[idx] && nums[idx + 1] > q[2] {
                        Self::add(&mut v, idx + 2, 1);
                    }
                }
                nums[idx] = q[2];
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_of_peaks() {
        assert_eq!(
            Solution::count_of_peaks(vec![3, 1, 4, 2, 5], vec_vec![[2, 3, 4], [1, 0, 4]]),
            [0]
        );
        assert_eq!(
            Solution::count_of_peaks(vec![4, 1, 4, 2, 1, 5], vec_vec![[2, 2, 4], [1, 0, 2], [1, 0, 4]]),
            [0, 1]
        );
    }
}
