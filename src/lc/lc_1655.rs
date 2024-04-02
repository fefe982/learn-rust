// https://leetcode.com/problems/distribute-repeating-integers/
// 1655. Distribute Repeating Integers
pub struct Solution;
impl Solution {
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        for n in nums {
            map.entry(n).and_modify(|e| *e += 1).or_insert(1);
        }
        let counts = map.values().cloned().collect::<Vec<_>>();
        let mut sums = vec![0; 1 << quantity.len()];
        for i in 1..1 << quantity.len() {
            sums[i] = sums[i & (i - 1)] + quantity[i.trailing_zeros() as usize];
        }
        let mut visited = vec![vec![false; 1 << quantity.len()]; counts.len()];
        let mut stack = vec![(0, (1 << quantity.len()) - 1, (1 << quantity.len()) - 1)];
        while let Some((i, mask_full, mask_next)) = stack.pop() {
            if mask_next == 0 {
                visited[i][mask_full] = true;
            } else {
                stack.push((i, mask_full, mask_full & (mask_next - 1)));
            }
            if sums[mask_next] <= counts[i] {
                if mask_next == mask_full {
                    return true;
                }
                if i + 1 < counts.len() {
                    let mask_n = mask_full & (!mask_next);
                    if !visited[i + 1][mask_n] {
                        stack.push((i + 1, mask_n, mask_n));
                    }
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_distribute() {
        assert_eq!(Solution::can_distribute(vec![1, 2, 3, 4], vec![2]), false);
        assert_eq!(Solution::can_distribute(vec![1, 2, 3, 3], vec![2]), true);
        assert_eq!(Solution::can_distribute(vec![1, 1, 2, 2], vec![2, 2]), true);
    }
}
