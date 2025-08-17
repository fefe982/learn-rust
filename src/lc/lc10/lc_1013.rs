// https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/
// 1013. Partition Array Into Three Parts With Equal Sum
pub struct Solution;
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut sum = arr.iter().sum::<i32>();
        if sum % 3 != 0 {
            return false;
        }
        sum /= 3;
        let mut cur = 0;
        let mut cnt = 0;
        for i in arr {
            cur += i;
            if cur == sum {
                cnt += 1;
                cur = 0;
            }
        }
        cnt >= 3
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_three_parts_equal_sum() {
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
            true
        );
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
            false
        );
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
            true
        );
    }
}
