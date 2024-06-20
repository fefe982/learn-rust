// https://leetcode.com/problems/number-of-beautiful-pairs/
// 2748. Number of Beautiful Pairs
pub struct Solution;
impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut first_count = vec![0; 10];
        let mut res = 0;
        for (idx, n) in nums.into_iter().enumerate() {
            let last = n % 10;
            let mut first = n;
            while first >= 10 {
                first /= 10;
            }
            res += match last {
                1 => idx as i32,
                2 | 4 | 8 => first_count[1] + first_count[3] + first_count[5] + first_count[7] + first_count[9],
                3 | 9 => idx as i32 - first_count[3] - first_count[6] - first_count[9],
                5 => idx as i32 - first_count[5],
                6 => first_count[1] + first_count[5] + first_count[7],
                7 => idx as i32 - first_count[7],
                _ => unreachable!(),
            };
            first_count[first as usize] += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_beautiful_pairs() {
        assert_eq!(Solution::count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
        assert_eq!(Solution::count_beautiful_pairs(vec![11, 21, 12]), 2);
    }
}
