// https://leetcode.com/problems/sort-the-jumbled-numbers/
// 2191. Sort the Jumbled Numbers
pub struct Solution;
impl Solution {
    fn map(num: i32, mapping: &Vec<i32>) -> i32 {
        if num == 0 {
            return mapping[0];
        }
        let mut n = 0;
        let mut num = num;
        let mut power = 1;
        while num > 0 {
            let d = (num % 10) as usize;
            n += mapping[d] * power;
            num /= 10;
            power *= 10;
        }
        n
    }
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums
            .into_iter()
            .map(|n| (n, Self::map(n, &mapping)))
            .collect::<Vec<_>>();
        nums.sort_by_key(|&(_, m)| m);
        nums.into_iter().map(|(n, _)| n).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_jumbled() {
        assert_eq!(
            Solution::sort_jumbled(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        assert_eq!(
            Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
            [338, 38, 991]
        );
        assert_eq!(
            Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
            [123, 456, 789]
        );
    }
}
