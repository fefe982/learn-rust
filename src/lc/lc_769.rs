// https://leetcode.com/problems/max-chunks-to-make-sorted/
// 769. Max Chunks To Make Sorted
pub struct Solution;
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut stk = vec![];
        for n in arr {
            if let Some(&t) = stk.last() {
                if t > n {
                    stk.pop();
                    while let Some(&t) = stk.last() {
                        if t <= n {
                            break;
                        }
                        stk.pop();
                    }
                    stk.push(t);
                } else {
                    stk.push(n);
                }
            } else {
                stk.push(n);
            }
        }
        stk.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_chunks_to_sorted() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
