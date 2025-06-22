// https://leetcode.com/problems/third-maximum-number/
// 414. Third Maximum Number
pub struct Solution;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut v = [i32::MIN; 3];
        let mut min = false;
        let mut c = 0;
        for n in nums {
            if n == i32::MIN {
                min = true;
                continue;
            }
            if n == v[0] || n == v[1] || n == v[2] {
                continue;
            }
            c += 1;
            if n > v[2] {
                v[2] = n;
                if n > v[1] {
                    v[2] = v[1];
                    v[1] = n;
                    if n > v[0] {
                        v[1] = v[0];
                        v[0] = n;
                    }
                }
            }
        }
        if min {
            c += 1;
        }
        if c >= 3 {
            v[2]
        } else {
            v[0]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn third_max() {
        assert_eq!(Solution::third_max(vec![1, 2, 2, 5, 3, 5]), 2);
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }
}
