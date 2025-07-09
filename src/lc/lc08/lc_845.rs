// https://leetcode.com/problems/longest-mountain-in-array/
// 845. Longest Mountain in Array
pub struct Solution;
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut max = 0;
        #[derive(PartialEq, Eq)]
        enum St {
            OUT,
            UP,
            DOWN,
        }
        let mut st = St::OUT;
        let mut m = 0;
        for i in 0..arr.len() - 1 {
            match st {
                St::OUT => {
                    if arr[i] < arr[i + 1] {
                        st = St::UP;
                        m = 2;
                    }
                }
                St::UP => match arr[i].cmp(&arr[i + 1]) {
                    std::cmp::Ordering::Greater => {
                        st = St::DOWN;
                        m += 1;
                    }
                    std::cmp::Ordering::Less => {
                        m += 1;
                    }
                    _ => st = St::OUT,
                },
                St::DOWN => match arr[i].cmp(&arr[i + 1]) {
                    std::cmp::Ordering::Greater => {
                        m += 1;
                    }
                    std::cmp::Ordering::Less => {
                        st = St::UP;
                        m = 2;
                    }
                    _ => st = St::OUT,
                },
            }
            if st == St::DOWN {
                max = max.max(m);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_mountain() {
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
        assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
    }
}
