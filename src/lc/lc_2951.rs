// https://leetcode.com/problems/find-the-peaks/
// 2951. Find the Peaks
pub struct Solution;
impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..mountain.len() - 1 {
            if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
                res.push(i as i32);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_peaks() {
        assert_eq!(Solution::find_peaks(vec![2, 4, 4]), []);
        assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), [1, 3]);
    }
}
