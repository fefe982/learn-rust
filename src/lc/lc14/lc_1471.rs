// https://leetcode.com/problems/the-k-strongest-values-in-an-array/
// 1471. The k Strongest Values in an Array
pub struct Solution;
impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_unstable();
        let im = (arr.len() - 1) / 2;
        let m = arr[im];
        let mut res = Vec::with_capacity(k as usize);
        let mut i = 0;
        let mut j = arr.len() - 1;
        while res.len() < k as usize {
            if j > im && i < im {
                if arr[j] - m >= m - arr[i] {
                    res.push(arr[j]);
                    j -= 1;
                } else {
                    res.push(arr[i]);
                    i += 1;
                }
            } else if j > im {
                res.push(arr[j]);
                j -= 1;
            } else if i < im {
                res.push(arr[i]);
                i += 1;
            } else {
                res.push(arr[im]);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_strongest() {
        assert_eq!(Solution::get_strongest(vec![-7, 22, 17, 3], 2), vec![22, 17]);
        assert_eq!(Solution::get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
        assert_eq!(Solution::get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
        assert_eq!(
            Solution::get_strongest(vec![6, 7, 11, 7, 6, 8], 5),
            vec![11, 8, 6, 6, 7]
        );
    }
}
