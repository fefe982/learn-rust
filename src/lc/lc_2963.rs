// https://leetcode.com/problems/count-the-number-of-good-partitions/
// 2963. Count the Number of Good Partitions
pub struct Solution;
impl Solution {
    pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::<i32, usize>::new();
        for (i, &n) in nums.iter().enumerate() {
            m.insert(n, i);
        }
        let mut cnt = 1;
        let mut e = 0;
        for (i, &n) in nums.iter().enumerate() {
            let ie = *m.get(&n).unwrap();
            if i > e {
                cnt = (cnt << 1) % 1_0000_0000_7i64;
            }
            if ie > e {
                e = ie;
            }
        }
        cnt as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_good_partitions() {
        assert_eq!(Solution::number_of_good_partitions(vec![1, 2, 3, 4]), 8);
        assert_eq!(Solution::number_of_good_partitions(vec![1, 1, 1, 1]), 1);
        assert_eq!(Solution::number_of_good_partitions(vec![1, 2, 1, 3]), 2);
    }
}
