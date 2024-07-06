// https://leetcode.com/problems/maximum-total-beauty-of-the-gardens/
// 2234. Maximum Total Beauty of the Gardens
pub struct Solution;
impl Solution {
    pub fn maximum_beauty(flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64 {
        let mut flowers = flowers;
        flowers.sort_unstable();
        let mut full_cnt = 0;
        while let Some(&n) = flowers.last() {
            if n >= target {
                flowers.pop();
                full_cnt += 1;
            } else {
                break;
            }
        }
        if flowers.is_empty() {
            return (full_cnt as i64) * (full as i64);
        }
        let mut pmap = vec![];
        let mut last = 0;
        let mut add = 0;
        for (i, &n) in flowers.iter().enumerate() {
            if n > last {
                add += (n - last) as i64 * (i as i64);
                pmap.push((add, n, i));
                if add >= new_flowers {
                    break;
                }
            }
            last = n;
        }
        let mut ans = 0;
        let mut full_flowers = 0;
        let mut partial_flowers = new_flowers;
        let mut map_idx = pmap.len() - 1;
        let mut bound = target;
        loop {
            while partial_flowers < pmap[map_idx].0 || pmap[map_idx].1 > bound {
                map_idx -= 1;
            }
            let mut partial_len = flowers.len() as i64;
            if map_idx + 1 < pmap.len() {
                partial_len = partial_len.min(pmap[map_idx + 1].2 as i64);
            }
            ans = ans.max(
                (full_cnt as i64) * (full as i64)
                    + if partial_len > 0 {
                        (pmap[map_idx].1 as i64 + (partial_flowers - pmap[map_idx].0) as i64 / partial_len)
                            .min(target as i64 - 1)
                            * (partial as i64)
                    } else {
                        0
                    },
            );
            if let Some(n) = flowers.pop() {
                full_flowers += (target - n) as i64;
                if full_flowers > new_flowers {
                    break;
                }
                full_cnt += 1;
                partial_flowers -= (target - n) as i64;
                bound = n;
            } else {
                break;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_maximum_beauty() {
        assert_eq!(
            Solution::maximum_beauty(vec![5, 19, 1, 1, 6, 10, 18, 12, 20, 10, 11], 6, 20, 3, 11),
            47
        );
        assert_eq!(Solution::maximum_beauty(vec![1, 3, 1, 1], 7, 6, 12, 1), 14);
        assert_eq!(Solution::maximum_beauty(vec![2, 4, 5, 3], 10, 5, 2, 6), 30);
    }
}
