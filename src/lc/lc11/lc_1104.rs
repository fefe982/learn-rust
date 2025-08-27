// https://leetcode.com/problems/path-in-zigzag-labelled-binary-tree/
// 1104. Path In Zigzag Labelled Binary Tree
pub struct Solution;
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut path = vec![];
        let mut l = label;
        let mut n = 1;
        while l > 0 {
            path.push(l);
            l /= 2;
            n *= 2;
        }
        n /= 2;
        for i in 0..path.len() / 2 {
            let j = 2 * i + 1;
            if j >= path.len() {
                break;
            }
            path[j] = n / 2 + n - 1 - path[j];
            n /= 4;
        }
        path.reverse();
        path
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_in_zig_zag_tree() {
        assert_eq!(Solution::path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
        assert_eq!(Solution::path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
    }
}
