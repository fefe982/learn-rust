// https://leetcode.com/problems/largest-time-for-given-digits
// 949. Largest Time for Given Digits
pub struct Solution;
impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut arr = arr;
        arr.sort_unstable();
        if arr[0] > 2 {
            return "".to_string();
        }
        if arr[0] == 2 && (arr[1] > 3 || arr[2] > 5) {
            return "".to_string();
        }
        if arr[1] > 5 {
            return "".to_string();
        }
        let mut res = "".to_string();

        for i in 0..4 {
            if arr[i] > 2 {
                break;
            }
            for j in 0..4 {
                if j == i {
                    continue;
                }
                if arr[i] * 10 + arr[j] > 23 {
                    break;
                }
                for k in 0..4 {
                    if k == i || k == j {
                        continue;
                    }
                    if arr[k] > 5 {
                        break;
                    }
                    for l in 0..4 {
                        if l == i || l == j || l == k {
                            continue;
                        }
                        res = format!("{}{}:{}{}", arr[i], arr[j], arr[k], arr[l]);
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_time_from_digits() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![1, 2, 3, 4]),
            "23:41".to_string()
        );
        assert_eq!(Solution::largest_time_from_digits(vec![5, 5, 5, 5]), "".to_string());
    }
}
