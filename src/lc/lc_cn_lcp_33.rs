// https://leetcode.cn/problems/o8SXZn/
// LCP 33. 蓄水
pub struct Solution;
impl Solution {
    pub fn store_water(mut bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        let mut cnt = 0;
        let mut res = i32::MAX;
        for (i, (b, &v)) in bucket.iter_mut().zip(vat.iter()).enumerate() {
            if v == 0 {
                continue;
            }
            if *b == 0 {
                cnt += 1;
                *b = 1
            }
            q.push(((v - 1) / *b + 1, i))
        }
        if q.is_empty() {
            return 0;
        }
        while cnt < res {
            let (c, i) = q.pop().unwrap();
            res = std::cmp::min(c + cnt, res);
            if c == 1 {
                break;
            }
            let t = (vat[i] - 1) / (c - 1) + 1;
            cnt += t - bucket[i];
            bucket[i] = t;
            q.push(((vat[i] - 1) / bucket[i] + 1, i));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn store_water() {
        assert_eq!(Solution::store_water(vec![1, 3], vec![6, 8]), 4);
        assert_eq!(Solution::store_water(vec![9, 0, 1], vec![0, 2, 2]), 3);
    }
}
