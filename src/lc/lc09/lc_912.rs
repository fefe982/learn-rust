// https://leetcode.com/problems/sort-an-array/
// 912. Sort an Array
pub struct Solution;
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            if nums[(i - 1) / 2] < nums[i] {
                let n = nums[i];
                let mut j = i;
                while j > 0 {
                    if nums[(j - 1) / 2] < n {
                        nums[j] = nums[(j - 1) / 2];
                    } else {
                        nums[j] = n;
                        break;
                    }
                    j = (j - 1) / 2;
                }
                if j == 0 {
                    nums[0] = n;
                }
            }
        }
        for i in (1..nums.len()).rev() {
            let save = nums[i];
            nums[i] = nums[0];
            let mut j = 0;
            while 2 * j + 1 < i {
                let mut nj = 2 * j + 1;
                if 2 * j + 2 < i && nums[2 * j + 2] > nums[nj] {
                    nj = 2 * j + 2;
                }
                if save >= nums[nj] {
                    nums[j] = save;
                    break;
                }
                nums[j] = nums[nj];
                j = nj;
            }
            if 2 * j + 1 >= i {
                nums[j] = save;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_array() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }
}
