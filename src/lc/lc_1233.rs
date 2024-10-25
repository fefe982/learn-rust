// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/
// 1233. Remove Sub-Folders from the Filesystem
pub struct Solution;
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort();
        let mut res = vec![folder[0].clone()];
        for i in 1..folder.len() {
            let last = res.last().unwrap();
            if !folder[i].starts_with(last) || folder[i].as_bytes()[last.len()] != b'/' {
                res.push(folder[i].clone())
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_remove_subfolders() {
        assert_eq!(
            Solution::remove_subfolders(vec_str!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]),
            vec_str!["/a", "/c/d", "/c/f"]
        );
        assert_eq!(
            Solution::remove_subfolders(vec_str!["/a", "/a/b/c", "/a/b/d"]),
            vec_str!["/a"]
        );
        assert_eq!(
            Solution::remove_subfolders(vec_str!["/a/b/c", "/a/b/ca", "/a/b/d"]),
            vec_str!["/a/b/c", "/a/b/ca", "/a/b/d"]
        );
    }
}
