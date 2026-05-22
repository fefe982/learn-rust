// https://leetcode.com/problems/minimum-operations-to-make-the-array-alternating/
// 2170. Minimum Operations to Make the Array Alternating
pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut count = [[0; 100_001]; 2];
        for (i, &num) in nums.iter().enumerate() {
            count[i % 2][num as usize] += 1;
        }
        let mut max1 = [0; 2];
        let mut max2 = [0; 2];
        for i in 0..2 {
            for j in 0..=100_000 {
                if count[i][j] > count[i][max1[i]] {
                    max2[i] = max1[i];
                    max1[i] = j;
                } else if count[i][j] > count[i][max2[i]] {
                    max2[i] = j;
                }
            }
        }
        if max1[0] != max1[1] {
            (nums.len() as i32) - count[0][max1[0]] - count[1][max1[1]]
        } else {
            (nums.len() as i32) - (count[0][max1[0]] + count[1][max2[1]]).max(count[0][max2[0]] + count[1][max1[1]])
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![3, 1, 3, 2, 4, 3]), 3);
        assert_eq!(Solution::minimum_operations(vec![1, 2, 2, 2, 2]), 2);
    }
}
