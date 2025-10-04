// https://leetcode.com/problems/cinema-seat-allocation/
// 1386. Cinema Seat Allocation
pub struct Solution;
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut reserved_seats = reserved_seats;
        reserved_seats.sort();
        let mut res = 0;
        let mut j = 0;
        let mut last_i = 0;
        while j < reserved_seats.len() {
            let i = reserved_seats[j][0];
            res += 2 * (i - last_i - 1);
            last_i = i;
            while j < reserved_seats.len() && reserved_seats[j][0] < i {
                j += 1;
            }
            if j >= reserved_seats.len() || reserved_seats[j][0] > i {
                res += 2;
                continue;
            }
            if reserved_seats[j][1] > 5
                || reserved_seats[j][1] == 1
                    && (j + 1 >= reserved_seats.len() || reserved_seats[j + 1][0] > i || reserved_seats[j + 1][1] > 5)
            {
                res += 1;
                while j < reserved_seats.len() && reserved_seats[j][0] == i && reserved_seats[j][1] < 6 {
                    j += 1;
                }
                if j >= reserved_seats.len() || reserved_seats[j][0] > i || reserved_seats[j][1] > 9 {
                    res += 1;
                }
            } else {
                while j < reserved_seats.len() && reserved_seats[j][0] == i && reserved_seats[j][1] < 4 {
                    j += 1;
                }
                if j >= reserved_seats.len() || reserved_seats[j][0] > i || reserved_seats[j][1] > 7 {
                    res += 1;
                } else {
                    while j < reserved_seats.len() && reserved_seats[j][0] == i && reserved_seats[j][1] < 6 {
                        j += 1;
                    }
                    if j >= reserved_seats.len() || reserved_seats[j][0] > i || reserved_seats[j][1] > 9 {
                        res += 1;
                    }
                }
            }
            while j < reserved_seats.len() && reserved_seats[j][0] == i {
                j += 1;
            }
        }
        res + 2 * (n - last_i)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_number_of_families() {
        assert_eq!(
            Solution::max_number_of_families(3, vec_vec![[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]]),
            4
        );
        assert_eq!(Solution::max_number_of_families(2, vec_vec![[2, 1], [1, 8], [2, 6]]), 2);
        assert_eq!(
            Solution::max_number_of_families(4, vec_vec![[4, 3], [1, 4], [4, 6], [1, 7]]),
            4
        );
    }
}
