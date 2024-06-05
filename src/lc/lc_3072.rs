// https://leetcode.com/problems/distribute-elements-into-two-arrays-ii/
// 3072. Distribute Elements Into Two Arrays II
pub struct Solution;
impl Solution {
    fn insert(v: &mut Vec<i32>, mut i: usize) {
        while i < v.len() {
            v[i] += 1;
            i += i & (!i + 1);
        }
    }
    fn get(v: &Vec<i32>, mut i: usize) -> i32 {
        let mut res = 0;
        while i > 0 {
            res += v[i];
            i &= i - 1;
        }
        res
    }
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_sort = nums.clone();
        nums_sort.sort_unstable();
        let mut m = std::collections::HashMap::new();
        for (num, i) in nums_sort.into_iter().zip(1..) {
            m.insert(num, i);
        }
        let mut r0 = vec![nums[0]];
        let mut v0 = vec![0; nums.len() + 1];
        Self::insert(&mut v0, m[&nums[0]]);
        let mut r1 = vec![nums[1]];
        let mut v1 = vec![0; nums.len() + 1];
        Self::insert(&mut v1, m[&nums[1]]);
        for num in nums.into_iter().skip(2) {
            let idx = m[&num];
            let c0 = r0.len() as i32 - Self::get(&v0, idx);
            let c1 = r1.len() as i32 - Self::get(&v1, idx);
            if c0 > c1 || (c0 == c1 && r0.len() <= r1.len()) {
                Self::insert(&mut v0, idx);
                r0.push(num);
            } else {
                Self::insert(&mut v1, idx);
                r1.push(num);
            }
        }
        r0.append(&mut r1);
        r0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_result_array() {
        assert_eq!(Solution::result_array(vec![2, 1, 3, 3]), [2, 3, 1, 3]);
        assert_eq!(Solution::result_array(vec![5, 14, 3, 1, 2]), [5, 3, 1, 2, 14]);
        assert_eq!(Solution::result_array(vec![3, 3, 3, 3]), [3, 3, 3, 3]);
    }
}
