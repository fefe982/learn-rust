// https://leetcode.com/problems/reverse-pairs/
// 493. Reverse Pairs
pub struct Solution;
impl Solution {
    fn merge(nums: &Vec<i32>, n: &mut Vec<i32>, s: usize, e1: usize, e2: usize) -> i32 {
        let e1 = e1.min(nums.len());
        let e2 = e2.min(nums.len());
        let mut cnt = 0;
        let mut s1 = s;
        let mut s2 = e1;
        while s1 < e1 && s2 < e2 {
            while s1 < e1 && nums[s1] as i64 > nums[s2] as i64 * 2 {
                cnt += e2 - s2;
                s1 += 1;
            }
            if s1 >= e1 {
                break;
            }
            while s2 < e2 && nums[s1] as i64 <= nums[s2] as i64 * 2 {
                s2 += 1;
            }
        }
        s1 = s;
        s2 = e1;
        let mut dst = s;
        while s1 < e1 && s2 < e2 {
            if nums[s1] > nums[s2] {
                n[dst] = nums[s1];
                s1 += 1;
            } else {
                n[dst] = nums[s2];
                s2 += 1;
            }
            dst += 1;
        }
        while s1 < e1 {
            n[dst] = nums[s1];
            dst += 1;
            s1 += 1;
        }
        while s2 < e2 {
            n[dst] = nums[s2];
            dst += 1;
            s2 += 1;
        }
        cnt as i32
    }
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut n = vec![0; nums.len()];
        let mut l = 1;
        let mut cnt = 0;
        while l < nums.len() {
            let mut s = 0;
            while s < nums.len() {
                let e1 = s + l;
                let e2 = e1 + l;
                cnt += Self::merge(&nums, &mut n, s, e1, e2);
                s = e2;
            }
            let tmp = nums;
            nums = n;
            n = tmp;
            l *= 2;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_pairs() {
        assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
        assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    }
}
