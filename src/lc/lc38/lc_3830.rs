// https://leetcode.com/problems/longest-alternating-subarray-after-removing-at-most-one-element/
// 3830. Longest Alternating Subarray After Removing at Most One Element
pub struct Solution;
impl Solution {
    pub fn longest_alternating(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pre = vec![1; n];
        if nums[1] != nums[0] {
            pre[1] = 2;
        }
        for i in 2..n {
            if (nums[i] > nums[i - 1] && nums[i - 1] < nums[i - 2])
                || (nums[i] < nums[i - 1] && nums[i - 1] > nums[i - 2])
            {
                pre[i] = pre[i - 1] + 1;
            } else if nums[i] != nums[i - 1] {
                pre[i] = 2;
            } else {
                pre[i] = 1;
            }
        }
        let mut suf = vec![1; n];
        let mut i = n - 1;
        while i > 0 {
            if pre[i] == 1 {
                i -= 1;
            } else {
                for j in i + 1 - pre[i]..i {
                    suf[j] = i + 1 - j;
                }
                i = i + 1 - pre[i];
            }
        }
        let mut ans = pre.iter().cloned().max().unwrap();
        fn ord(x: i32, y: i32) -> i32 {
            match x.cmp(&y) {
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => -1,
            }
        }
        for i in 1..n - 1 {
            if nums[i - 1] == nums[i + 1] {
                continue;
            }
            let x = if i == 1 { 0 } else { ord(nums[i - 2], nums[i - 1]) };
            let y = ord(nums[i - 1], nums[i + 1]);
            let z = if i == n - 2 { 0 } else { ord(nums[i + 1], nums[i + 2]) };
            if x == -y && y == -z {
                ans = ans.max(pre[i - 1] + suf[i + 1]);
            } else if x == -y {
                ans = ans.max(pre[i - 1] + 1)
            } else if y == -z {
                ans = ans.max(suf[i + 1] + 1)
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_alternating() {
        assert_eq!(
            Solution::longest_alternating(vec![
                99347, 55126, 5307, 33937, 67014, 63692, 53076, 39756, 62469, 46931, 76466, 28632, 66151, 18255, 36942,
                18317, 99065
            ]),
            11
        );
        assert_eq!(Solution::longest_alternating(vec![2, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_alternating(vec![3, 2, 1, 2, 3, 2, 1]), 4);
        assert_eq!(Solution::longest_alternating(vec![100000, 100000]), 1);
    }
}
