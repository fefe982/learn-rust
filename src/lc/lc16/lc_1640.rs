// https://leetcode.com/problems/check-array-formation-through-concatenation/
// 1640. Check Array Formation Through Concatenation
pub struct Solution;
impl Solution {
    fn check(arr: &Vec<i32>, pieces: &Vec<Vec<i32>>, idx: usize, cache: &mut Vec<Option<bool>>) -> bool {
        if idx == arr.len() {
            return true;
        }
        if let Some(c) = cache[idx] {
            return c;
        }
        for piece in pieces {
            if idx + piece.len() > arr.len() {
                continue;
            }
            if arr[idx..idx + piece.len()] == *piece {
                if Self::check(arr, pieces, idx + piece.len(), cache) {
                    cache[idx] = Some(true);
                    return true;
                }
            }
        }
        cache[idx] = Some(false);
        false
    }
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        Self::check(&arr, &pieces, 0, &mut vec![None; arr.len()])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_form_array() {
        assert_eq!(Solution::can_form_array(vec![15, 88], vec_vec![[88], [15]]), true);
        assert_eq!(
            Solution::can_form_array(vec![49, 18, 16], vec_vec![[16, 18, 49]]),
            false
        );
        assert_eq!(
            Solution::can_form_array(vec![91, 4, 64, 78], vec_vec![[78], [4, 64], [91]]),
            true
        );
    }
}
