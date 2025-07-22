// https://leetcode.com/problems/add-to-array-form-of-integer/
// 989. Add to Array-Form of Integer

pub struct Solution;
impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut carry = k;
        for mut n in num.into_iter().rev() {
            n += carry;
            carry = n / 10;
            res.push(n % 10);
        }
        while carry > 0 {
            res.push(carry % 10);
            carry = carry / 10;
        }
        res.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_to_array_form() {
        assert_eq!(
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::add_to_array_form(vec![2, 7, 4], 181),
            vec![4, 5, 5]
        );
        assert_eq!(
            Solution::add_to_array_form(vec![2, 1, 5], 806),
            vec![1, 0, 2, 1]
        );
    }
}
