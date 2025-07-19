// https://leetcode.com/problems/fair-candy-swap/
// 888. Fair Candy Swap
pub struct Solution;
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let mut alice_sizes = alice_sizes;
        let mut bob_sizes = bob_sizes;
        alice_sizes.sort_unstable();
        bob_sizes.sort_unstable();
        let alice_sum = alice_sizes.iter().sum::<i32>();
        let bob_sum = bob_sizes.iter().sum::<i32>();
        let diff = (bob_sum - alice_sum) / 2;
        let mut i = 0;
        for j in bob_sizes {
            while i < alice_sizes.len() && alice_sizes[i] + diff < j {
                i += 1;
            }
            if i == alice_sizes.len() {
                break;
            }
            if alice_sizes[i] + diff == j {
                return vec![alice_sizes[i], j];
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>, expect: Vec<i32>) {
        let ans = Solution::fair_candy_swap(alice_sizes.clone(), bob_sizes.clone());
        assert_eq!(ans[0] - ans[1], expect[0] - expect[1]);
        assert!(alice_sizes.iter().find(|&&x| x == ans[0]).is_some());
        assert!(bob_sizes.iter().find(|&&x| x == ans[1]).is_some());
    }
    #[test]
    fn fair_candy_swap() {
        check(vec![1, 1], vec![2, 2], vec![1, 2]);
        check(vec![1, 2], vec![2, 3], vec![1, 2]);
        check(vec![2], vec![1, 3], vec![2, 3]);
    }
}
