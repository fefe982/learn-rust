// https://leetcode.com/problems/adding-two-negabinary-numbers/
// 1073. Adding Two Negabinary Numbers
pub struct Solution;
impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let l = std::cmp::max(arr1.len(), arr2.len());
        let mut rev = Vec::new();
        let mut carry: i32 = 0;
        for (&i, &j) in arr1
            .iter()
            .rev()
            .chain(std::iter::repeat(&0))
            .take(l)
            .zip(arr2.iter().rev().chain(std::iter::repeat(&0)).take(l))
        {
            carry = match i + j + carry {
                -1 => {
                    rev.push(1);
                    1
                }
                0 => {
                    rev.push(0);
                    0
                }
                1 => {
                    rev.push(1);
                    0
                }
                2 => {
                    rev.push(0);
                    -1
                }
                3 => {
                    rev.push(1);
                    -1
                }
                _ => {
                    panic!("overflow")
                }
            }
        }
        if carry == 1 {
            rev.push(1);
        } else if carry == -1 {
            rev.push(1);
            rev.push(1);
        } else {
            let mut l = 0;
            for idx in (0..rev.len()).rev() {
                if rev[idx] == 1 {
                    l = idx;
                    break;
                }
            }
            rev.resize(l + 1, 0);
        }
        rev.reverse();
        rev
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_negabinary() {
        assert_eq!(
            Solution::add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1]),
            vec![1, 0, 0, 0, 0]
        );
        assert_eq!(Solution::add_negabinary(vec![0], vec![0]), vec![0]);
        assert_eq!(Solution::add_negabinary(vec![0], vec![1]), vec![1]);
    }
}
