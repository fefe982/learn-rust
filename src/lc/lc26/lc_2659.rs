// https://leetcode.com/problems/make-array-empty/
// 2659. Make Array Empty
pub struct Solution;
impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().zip(1..).collect::<Vec<_>>();
        nums.sort_by_key(|&(i, _)| std::cmp::Reverse(i));
        let mut cnt = vec![0; nums.len() + 1];
        let add = |cnt: &mut Vec<i32>, mut n: usize| {
            while n < cnt.len() {
                cnt[n] += 1;
                n += n & (!n + 1);
            }
        };
        let get = |cnt: &Vec<i32>, mut n: usize| {
            let mut ans = 0;
            while n > 0 {
                ans += cnt[n];
                n -= n & (!n + 1);
            }
            ans
        };
        add(&mut cnt, nums[0].1);
        let mut last = nums[0].1;
        let mut ans = 1;
        for i in 1..nums.len() {
            add(&mut cnt, nums[i].1);
            if nums[i].1 < last {
                ans += (get(&cnt, last) - get(&cnt, nums[i].1)) as i64;
            } else {
                ans += i as i64 + 1 - get(&cnt, nums[i].1) as i64 + get(&cnt, last) as i64;
            }
            last = nums[i].1;
        }
        ans + nums[nums.len() - 1].1 as i64 - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_operations_to_empty_array() {
        assert_eq!(Solution::count_operations_to_empty_array(vec![3, 4, -1]), 5);
        assert_eq!(Solution::count_operations_to_empty_array(vec![1, 2, 4, 3]), 5);
        assert_eq!(Solution::count_operations_to_empty_array(vec![1, 2, 3]), 3);
    }
}
