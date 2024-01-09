// https://leetcode.com/problems/three-equal-parts/
// 927. Three Equal Parts
pub struct Solution;
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let mut one_pos = vec![];
        let arr_len = arr.len();
        for (i, v) in arr.into_iter().enumerate() {
            if v == 1 {
                one_pos.push(i);
            }
        }
        if one_pos.len() % 3 != 0 {
            return vec![-1, -1];
        }
        if one_pos.len() == 0 {
            return vec![0, 2];
        }
        let trailing_zero = arr_len - one_pos[one_pos.len() - 1];
        let n1 = one_pos.len() / 3;
        if one_pos[n1] - one_pos[n1 - 1] < trailing_zero || one_pos[n1 * 2] - one_pos[n1 * 2 - 1] < trailing_zero {
            return vec![-1, -1];
        }
        for i in 1..n1 {
            if one_pos[n1 + i] - one_pos[n1] != one_pos[i] - one_pos[0]
                || one_pos[n1 * 2 + i] - one_pos[n1 * 2] != one_pos[i] - one_pos[0]
            {
                return vec![-1, -1];
            }
        }
        vec![
            (one_pos[n1 - 1] + trailing_zero - 1) as i32,
            (one_pos[n1 * 2 - 1] + trailing_zero) as i32,
        ]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_three_equal_parts() {
        assert_eq!(Solution::three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
        assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 1, 1]), vec![-1, -1]);
        assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 1]), vec![0, 2]);
    }
}
