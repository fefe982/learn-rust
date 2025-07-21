// https://leetcode.com/problems/arithmetic-subarrays/
// 1630. Arithmetic Subarrays
pub struct Solution;
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut result: Vec<bool> = Vec::with_capacity(l.len());
        for (&l_idx, &r_idx) in l.iter().zip(r.iter()) {
            let l = l_idx as usize;
            let r = r_idx as usize;
            if l + 1 == r {
                result.push(true);
                continue;
            }
            let mut min = i32::MAX;
            let mut max = i32::MIN;
            for &n in nums[l..=r].iter() {
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
            }
            if min == max {
                result.push(true);
                continue;
            }
            if (max - min) % (r - l) as i32 != 0 {
                result.push(false);
                continue;
            }
            let d = (max - min) / (r - l) as i32;
            let mut met = vec![false; r - l + 1];
            let mut good = true;
            for &n in nums[l..=r].iter() {
                if (n - min) % d != 0 {
                    good = false;
                    break;
                }
                let i = ((n - min) / d) as usize;
                if met[i] {
                    good = false;
                    break;
                }
                met[i] = true;
            }
            result.push(good);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_arithmetic_subarrays() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            ),
            vec![true, false, true]
        );
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            ),
            vec![false, true, false, false, true, true]
        );
    }
}
