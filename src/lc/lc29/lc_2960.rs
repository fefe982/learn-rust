// https://leetcode.com/problems/count-tested-devices-after-test-operations/
// 2960. Count Tested Devices After Test Operations
pub struct Solution;
impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        battery_percentages
            .into_iter()
            .fold(0, |acc, p| if p > acc { acc + 1 } else { acc })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_tested_devices() {
        assert_eq!(Solution::count_tested_devices(vec![1, 1, 2, 1, 3]), 3);
        assert_eq!(Solution::count_tested_devices(vec![0, 1, 2]), 2);
    }
}
