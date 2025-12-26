// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/
// 2948. Make Array Lexicographically Smallest by Reversing Subarrays
pub struct Solution;
impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut ni = nums.iter().cloned().enumerate().collect::<Vec<_>>();
        let mut res = nums;
        let mut i = 0;
        ni.sort_unstable_by_key(|v| v.1);
        while i < ni.len() {
            let mut j = i;
            let mut v = vec![ni[i].0];
            while j + 1 < ni.len() && ni[j + 1].1 - ni[j].1 <= limit {
                j += 1;
                v.push(ni[j].0);
            }
            v.sort();
            for k in 0..v.len() {
                res[v[k]] = ni[i + k].1;
            }
            i = j + 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexicographically_smallest_array() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            [1, 3, 5, 8, 9]
        );
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
            [1, 6, 7, 18, 1, 2]
        );
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
            [1, 7, 28, 19, 10]
        );
    }
}
