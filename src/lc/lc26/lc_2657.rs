// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/
// 2657. Find the Prefix Common Array of Two Arrays
pub struct Solution;
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut cnt = vec![0; a.len() + 1];
        let mut res = Vec::with_capacity(a.len());
        let mut c = 0;
        for (aa, bb) in a.into_iter().zip(b.into_iter()) {
            cnt[aa as usize] += 1;
            if cnt[aa as usize] == 2 {
                c += 1;
            }
            cnt[bb as usize] += 1;
            if cnt[bb as usize] == 2 {
                c += 1;
            }
            res.push(c);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_the_prefix_common_array() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 2, 3, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        );
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 2, 3], vec![3, 1, 2]),
            vec![0, 1, 3]
        );
    }
}
