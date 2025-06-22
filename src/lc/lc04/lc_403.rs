// https://leetcode.com/problems/frog-jump/
// 403. Frog Jump
pub struct Solution;
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut map = std::collections::BTreeMap::<i32, std::collections::BTreeSet<i32>>::new();
        for &s in &stones {
            map.insert(s, std::collections::BTreeSet::new());
        }
        map.entry(0).or_default().insert(0);
        for i in 0..stones.len() - 1 {
            let s = stones[i];
            let cdist = map.remove(&s).unwrap();
            for dist in cdist {
                if dist > 1 {
                    map.entry(s + dist - 1).and_modify(|x| {
                        x.insert(dist - 1);
                    });
                }
                map.entry(s + dist).and_modify(|x| {
                    x.insert(dist);
                });
                map.entry(s + dist + 1).and_modify(|x| {
                    x.insert(dist + 1);
                });
            }
        }
        !map.get(stones.last().unwrap()).unwrap().is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_cross() {
        assert_eq!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);
        assert_eq!(Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);
    }
}
