// https://leetcode.com/problems/maximum-number-of-groups-getting-fresh-donuts/
// 1815. Maximum Number of Groups Getting Fresh Donuts
pub struct Solution;
impl Solution {
    fn search(
        cnt: &mut Vec<i32>,
        batch_size: usize,
        memory: &mut std::collections::HashMap<Vec<i32>, i32>,
        left: usize,
    ) -> i32 {
        if let Some(&res) = memory.get(cnt) {
            return res;
        }
        let mut res = 0;
        for i in 1..batch_size {
            if cnt[i] > 0 {
                cnt[i] -= 1;
                res = res.max(
                    Self::search(cnt, batch_size, memory, (left + batch_size - i) % batch_size)
                        + if left == 0 { 1 } else { 0 },
                );
                cnt[i] += 1;
            }
        }
        memory.insert(cnt.clone(), res);
        res
    }
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        let batch_size = batch_size as usize;
        let mut cnt = vec![0; batch_size as usize];
        for g in groups {
            cnt[g as usize % batch_size] += 1;
        }
        let mut res = cnt[0];
        for i in 1..batch_size / 2 {
            if batch_size - i != i {
                let add = cnt[i].min(cnt[batch_size - i]);
                res += add;
                cnt[i] -= add;
                cnt[batch_size - i] -= add;
            } else {
                let add = cnt[i] / 2;
                res += add;
                cnt[i] -= add * 2;
            }
        }
        res += Self::search(&mut cnt, batch_size, &mut std::collections::HashMap::new(), 0);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_happy_groups() {
        assert_eq!(Solution::max_happy_groups(3, vec![1, 2, 3, 4, 5, 6]), 4);
        assert_eq!(Solution::max_happy_groups(4, vec![1, 3, 2, 5, 2, 2, 1, 6]), 4);
    }
}
