// https://leetcode.com/problems/subsets-ii/
// 90. Subsets II
pub struct Solution;
impl Solution {
    fn get_subset(v: &mut Vec<i32>, m: &Vec<i32>, idx: usize, ret: &mut Vec<Vec<i32>>) {
        if idx == 21 {
            return;
        }
        Self::get_subset(v, m, idx + 1, ret);
        for _ in 0..m[idx] {
            v.push(idx as i32 - 10);
            ret.push(v.clone());
            Self::get_subset(v, m, idx + 1, ret);
        }
        v.truncate(v.len() - m[idx] as usize);
    }
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut m = vec![0; 21];
        for n in nums {
            m[(n + 10) as usize] += 1;
        }
        let mut ret = vec![vec![]];
        Self::get_subset(&mut vec![], &m, 0, &mut ret);
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_subsets_with_dup() {
        assert_sort_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec_vec![[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]
        );
        assert_sort_eq!(Solution::subsets_with_dup(vec![0]), vec_vec![[], [0]]);
    }
}
