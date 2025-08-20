// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
// 1502. Can Make Arithmetic Progression From Sequence
pub struct Solution;
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        if arr.len() <= 2 {
            return true;
        }
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &n in &arr {
            min = std::cmp::min(min, n);
            max = std::cmp::max(max, n);
        }
        if max == min {
            return true;
        }
        let l = arr.len() as i32;
        if (max - min) % (l - 1) != 0 {
            return false;
        }
        let g = (max - min) / (l - 1);
        let mut v = vec![false; arr.len()];
        for n in arr {
            if (n - min) % g != 0 {
                return false;
            }
            let i = ((n - min) / g) as usize;
            if v[i] {
                return false;
            }
            v[i] = true;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_make_arithmetic_progression() {
        assert_eq!(
            Solution::can_make_arithmetic_progression(vec![3, 5, 1]),
            true
        );
        assert_eq!(
            Solution::can_make_arithmetic_progression(vec![1, 2, 4]),
            false
        );
    }
}
