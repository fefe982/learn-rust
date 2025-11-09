// https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/
// 3542. Minimum Operations to Convert All Elements to Zero
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut s = vec![];
        let mut ans = 0;
        for n in nums {
            while let Some(x) = s.last() {
                match n.cmp(x) {
                    std::cmp::Ordering::Less => {
                        s.pop();
                    }
                    std::cmp::Ordering::Equal => {
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        ans += 1;
                        s.push(n);
                        break;
                    }
                }
            }
            if s.is_empty() && n != 0 {
                ans += 1;
                s.push(n);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![0, 2]), 1);
        assert_eq!(Solution::min_operations(vec![3, 1, 2, 1]), 3);
    }
}
