// https://leetcode.com/problems/fruit-into-baskets
// 904. Fruit Into Baskets
pub struct Solution;
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut fruit = [-1, -1];
        let mut count = [0, 0];
        let mut s = 0;
        for e in 0..fruits.len() {
            if fruits[e] == fruit[0] || fruits[e] == fruit[1] {
                if fruits[e] == fruit[0] {
                    count[0] += 1;
                } else {
                    count[1] += 1;
                }
            } else if (fruit[0] == -1) || (fruit[1] == -1) {
                if fruit[0] == -1 {
                    fruit[0] = fruits[e];
                    count[0] = 1;
                } else {
                    fruit[1] = fruits[e];
                    count[1] = 1;
                }
            } else {
                max = max.max(count[0] + count[1]);
                loop {
                    s += 1;
                    if fruits[s - 1] == fruit[0] {
                        count[0] -= 1;
                        if count[0] == 0 {
                            fruit[0] = fruits[e];
                            count[0] = 1;
                            break;
                        }
                    } else {
                        count[1] -= 1;
                        if count[1] == 0 {
                            fruit[1] = fruits[e];
                            count[1] = 1;
                            break;
                        }
                    }
                }
            }
        }
        max.max(count[0] + count[1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_fruit() {
        assert_eq!(Solution::total_fruit(vec![6, 2, 1, 1, 3, 6, 6]), 3);
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }
}
