// https://leetcode.com/problems/number-of-adjacent-elements-with-the-same-color/
// 2672. Number of Adjacent Elements With the Same Color
pub struct Solution;
impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = vec![0; n as usize + 2];
        let mut res = Vec::with_capacity(queries.len());
        let mut c = 0;
        for q in queries {
            let i = q[0] as usize + 1;
            if arr[i] != 0 {
                if arr[i] == arr[i - 1] {
                    c -= 1;
                }
                if arr[i] == arr[i + 1] {
                    c -= 1;
                }
            }
            if q[1] == arr[i - 1] {
                c += 1;
            }
            if q[1] == arr[i + 1] {
                c += 1;
            }
            arr[i] = q[1];
            res.push(c);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn color_the_array() {
        assert_eq!(
            Solution::color_the_array(4, vec_vec![[0, 2], [1, 2], [3, 1], [1, 1], [2, 1]]),
            vec![0, 1, 1, 0, 2]
        );
        assert_eq!(Solution::color_the_array(1, vec_vec![[0, 100000]]), vec![0]);
    }
}
