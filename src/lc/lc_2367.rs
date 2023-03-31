// https://leetcode.com/problems/number-of-arithmetic-triplets/
// 2367. Number of Arithmetic Triplets
pub struct Solution;
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut i = 0;
        let mut j = i + 1;
        let mut k = j + 1;
        let mut cnt = 0;
        while i < nums.len() && j < nums.len() && k < nums.len() {
            if nums[j] - nums[i] < diff {
                j += 1;
                continue;
            }
            if nums[j] - nums[i] > diff {
                i += 1;
                continue;
            }
            if nums[k] - nums[j] < diff {
                k += 1;
                continue;
            }
            if nums[k] - nums[j] > diff {
                i += 1;
                continue;
            }
            cnt += 1;
            i += 1;
            j += 1;
            k += 1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arithmetic_triplets() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
        assert_eq!(Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
