// https://leetcode.com/problems/intersection-of-multiple-arrays/
// 2248. Intersection of Multiple Arrays
pub struct Solution;
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut count = std::collections::HashMap::new();
        for arr in nums {
            for &num in &arr {
                *count.entry(num).or_insert(0) += 1;
            }
        }
        let mut v = count
            .into_iter()
            .filter_map(|(num, c)| if c == n { Some(num) } else { None })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_intersection() {
        assert_eq!(
            Solution::intersection(vec_vec![[3, 1, 2, 4, 5], [1, 2, 3, 4], [3, 4, 5, 6]]),
            vec![3, 4]
        );
        assert_eq!(
            Solution::intersection(vec_vec![[1, 2, 3], [4, 5, 6]]),
            Vec::<i32>::new()
        );
    }
}
