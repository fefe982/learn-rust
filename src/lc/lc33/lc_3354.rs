// https://leetcode.com/problems/make-array-elements-equal-to-zero/
// 3354. Make Array Elements Equal to Zero
pub struct Solution;
impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut sums = Vec::with_capacity(nums.len() + 1);
        sums.push(0);
        for i in 0..nums.len() {
            sums.push(sums[i] + nums[i]);
        }
        let sum = *sums.last().unwrap();
        let mut mid = sum / 2;
        let pos1 = sums.partition_point(|&x| x < mid);
        let pos2 = sums.partition_point(|&x| x <= mid);
        let mut cnt = ((pos2 - pos1) as i32).max(1) - 1;
        if sum % 2 == 1 {
            mid += 1;
            let pos1 = sums.partition_point(|&x| x < mid);
            let pos2 = sums.partition_point(|&x| x <= mid);
            cnt += ((pos2 - pos1) as i32).max(1) - 1;
        } else {
            cnt *= 2;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_valid_selections() {
        assert_eq!(Solution::count_valid_selections(vec![0, 1]), 1);
        assert_eq!(Solution::count_valid_selections(vec![0]), 2);
        assert_eq!(
            Solution::count_valid_selections(vec![16, 13, 10, 0, 0, 0, 10, 6, 7, 8, 7]),
            3
        );
        assert_eq!(Solution::count_valid_selections(vec![1, 0, 2, 0, 3]), 2);
        assert_eq!(Solution::count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]), 0);
    }
}
