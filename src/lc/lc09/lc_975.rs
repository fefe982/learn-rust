// https://leetcode.com/problems/odd-even-jump/
// 975. Odd Even Jump
pub struct Solution;
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut m = std::collections::BTreeMap::<i32, usize>::new();
        let mut odd = vec![false; n];
        let mut even = vec![false; n];
        even[n - 1] = true;
        odd[n - 1] = true;
        m.insert(arr[n - 1], n - 1);
        for i in (0..n - 1).rev() {
            if let Some((_, &ni)) = m.range(arr[i]..).next() {
                odd[i] = even[ni];
            }
            if let Some((_, &ni)) = m.range(..=arr[i]).rev().next() {
                even[i] = odd[ni];
            }
            m.insert(arr[i], i);
        }
        println!("{odd:?}");
        println!("{even:?}");
        odd.into_iter().filter(|&x| x).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_odd_even_jumps() {
        assert_eq!(Solution::odd_even_jumps(vec![5, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::odd_even_jumps(vec![10, 13, 12, 14, 15]), 2);
        assert_eq!(Solution::odd_even_jumps(vec![2, 3, 1, 1, 4]), 3);
    }
}
