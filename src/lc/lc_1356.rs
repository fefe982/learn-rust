// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
// 1356. Sort Integers by The Number of 1 Bits
pub struct Solution;
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_by(|&a, &b| {
            let ac = a.count_ones();
            let bc = b.count_ones();
            (ac, a).cmp(&(bc, b))
        });
        arr
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_bits() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
