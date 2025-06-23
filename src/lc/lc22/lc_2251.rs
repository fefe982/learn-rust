// https://leetcode.com/problems/number-of-flowers-in-full-bloom/
// 2251. Number of Flowers in Full Bloom
pub struct Solution;
impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut t0 = Vec::with_capacity(flowers.len());
        let mut t1 = Vec::with_capacity(flowers.len());
        for flower in flowers {
            t0.push(flower[0]);
            t1.push(flower[1]);
        }
        t0.sort_unstable();
        t1.sort_unstable();
        let mut ans = Vec::with_capacity(people.len());
        for p in people {
            ans.push((t0.partition_point(|&x| x <= p) - t1.partition_point(|&x| x < p)) as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_full_bloom_flowers() {
        assert_eq!(
            Solution::full_bloom_flowers(
                vec_vec![[1, 6], [3, 7], [9, 12], [4, 13]],
                vec![2, 3, 7, 11]
            ),
            [1, 2, 2, 2]
        );
        assert_eq!(
            Solution::full_bloom_flowers(vec_vec![[1, 10], [3, 3]], vec![3, 3, 2]),
            vec![2, 2, 1]
        );
    }
}
