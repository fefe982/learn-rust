// https://leetcode.com/problems/longest-absolute-file-path/
// 388. Longest Absolute File Path
pub struct Solution;
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let lines = input.split('\n').collect::<Vec<_>>();
        let mut stack = vec![0];
        let mut max = 0;
        for line in lines {
            let mut line = line;
            let mut depth = 1;
            while line.starts_with('\t') {
                line = &line[1..];
                depth += 1;
            }
            while stack.len() > depth {
                stack.pop();
            }
            if stack.len() <= depth {
                stack.push(0);
            }
            stack[depth] = stack[depth - 1] + line.len() + 1;
            if line.contains('.') {
                max = max.max(stack[depth] - 1);
            }
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_longest_path() {
        assert_eq!(
            Solution::length_longest_path(String::from("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext")),
            20
        );
        assert_eq!(Solution::length_longest_path(String::from("a")), 0);
    }
}
