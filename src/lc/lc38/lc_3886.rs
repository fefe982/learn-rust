// https://leetcode.com/problems/sum-of-sortable-integers/
// 3886. Sum of Sortable Integers
pub struct Solution;
impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut desc_prefix = vec![0usize; n];
        for i in 0..n.saturating_sub(1) {
            desc_prefix[i + 1] = desc_prefix[i] + usize::from(nums[i] > nums[i + 1]);
        }

        let mut next_desc = vec![n; n + 1];
        for i in (0..n.saturating_sub(1)).rev() {
            next_desc[i] = if nums[i] > nums[i + 1] { i } else { next_desc[i + 1] };
        }

        fn can_sort_with_k(arr: &[i32], k: usize, desc_prefix: &[usize], next_desc: &[usize]) -> bool {
            let n = arr.len();
            let blocks = n / k;
            let mut prev_max = i32::MIN;

            for b in 0..blocks {
                let start = b * k;
                let end = start + k;

                let internal_desc = desc_prefix[end - 1] - desc_prefix[start];
                let wrap_desc = usize::from(arr[end - 1] > arr[start]);
                let total_desc = internal_desc + wrap_desc;
                if total_desc > 1 {
                    return false;
                }

                let (cur_min, cur_max) = if total_desc == 0 {
                    (arr[start], arr[start])
                } else if internal_desc == 1 {
                    let p = next_desc[start]; // p in [start, end - 2]
                    (arr[p + 1], arr[p])
                } else {
                    // The only descent is at the circular boundary end-1 -> start.
                    (arr[start], arr[end - 1])
                };

                if b > 0 && prev_max > cur_min {
                    return false;
                }
                prev_max = cur_max;
            }
            true
        }

        let mut divisors = Vec::new();
        let mut d = 1usize;
        while d * d <= n {
            if n % d == 0 {
                divisors.push(d);
                if d * d != n {
                    divisors.push(n / d);
                }
            }
            d += 1;
        }

        let mut ans = 0;
        for k in divisors {
            if can_sort_with_k(&nums, k, &desc_prefix, &next_desc) {
                ans += k;
            }
        }

        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sortable_integers() {
        assert_eq!(Solution::sortable_integers(vec![3, 1, 2]), 3);
        assert_eq!(Solution::sortable_integers(vec![7, 6, 5]), 0);
        assert_eq!(Solution::sortable_integers(vec![5, 8]), 3);
    }
}
