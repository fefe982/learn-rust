// https://leetcode.com/problems/execution-of-all-suffix-instructions-staying-in-a-grid/
// 2120. Execution of All Suffix Instructions Staying in a Grid
pub struct Solution;
impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let m = s.len();
        let bytes = s.as_bytes();
        let mut ans = vec![0; m];

        for i in 0..m {
            let mut r = start_pos[0];
            let mut c = start_pos[1];
            let mut steps = 0;

            for &ch in bytes.iter().skip(i) {
                match ch {
                    b'L' => c -= 1,
                    b'R' => c += 1,
                    b'U' => r -= 1,
                    b'D' => r += 1,
                    _ => unreachable!(),
                }

                if r < 0 || r >= n || c < 0 || c >= n {
                    break;
                }

                steps += 1;
            }

            ans[i] = steps;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_execute_instructions() {
        assert_eq!(
            Solution::execute_instructions(3, vec![0, 1], "RRDDLU".to_string()),
            vec![1, 5, 4, 3, 1, 0]
        );
        assert_eq!(
            Solution::execute_instructions(2, vec![1, 1], "LURD".to_string()),
            vec![4, 1, 0, 0]
        );
        assert_eq!(
            Solution::execute_instructions(1, vec![0, 0], "LRUD".to_string()),
            vec![0, 0, 0, 0]
        );
    }
}
