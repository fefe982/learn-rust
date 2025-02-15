// https://leetcode.com/problems/rotate-array/
// 189. Rotate Array
pub struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (nums.len() - k as usize % nums.len()) % nums.len();
        if k == 0 {
            return;
        }
        let mut flen = k;
        if nums.len() - k < flen {
            flen = nums.len() - k;
        }
        let mut flag = vec![false; flen];
        for i in 0..flen {
            if flag[i] {
                continue;
            }
            let mut j = i;
            let v = nums[i];
            loop {
                if j < flen {
                    flag[j] = true;
                }
                let next = (j + k) % nums.len();
                if next == i {
                    nums[j] = v;
                    break;
                }
                nums[j] = nums[next];
                j = next;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(mut nums: Vec<i32>, k: i32, expect: Vec<i32>) {
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, expect);
    }
    #[test]
    fn rotate() {
        check(vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]);
        check(vec![-1, -100, 3, 99], 2, vec![3, 99, -1, -100]);
    }
}
