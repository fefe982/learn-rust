// https://leetcode.com/problems/maximum-distance-in-arrays/
// 624. Maximum Distance in Arrays
pub struct Solution;
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min = ((i32::MAX, 0), (i32::MAX, 0));
        let mut max = ((i32::MIN, 0), (i32::MIN, 0));
        for (i, a) in arrays.into_iter().enumerate() {
            if a[0] < min.0 .0 {
                min = ((a[0], i), min.0);
            } else if a[0] < min.1 .0 {
                min = (min.0, (a[0], i));
            }
            if a[a.len() - 1] > max.0 .0 {
                max = ((a[a.len() - 1], i), max.0);
            } else if a[a.len() - 1] > max.1 .0 {
                max = (max.0, (a[a.len() - 1], i));
            }
        }
        if min.0 .1 != max.0 .1 {
            max.0 .0 - min.0 .0
        } else {
            (max.1 .0 - min.0 .0).max(max.0 .0 - min.1 .0)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_distance() {
        assert_eq!(Solution::max_distance(vec_vec![[1, 2, 3], [4, 5], [1, 2, 3]]), 4);
        assert_eq!(Solution::max_distance(vec_vec![[1], [1]]), 0);
    }
}
