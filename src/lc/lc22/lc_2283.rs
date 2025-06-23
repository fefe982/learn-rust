// https://leetcode.com/problems/minimum-hours-of-training-to-win-a-competition/
// 2383. Minimum Hours of Training to Win a Competition
pub struct Solution;
impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut eng = initial_energy;
        let mut exp = initial_experience;
        let mut train = 0;
        for (&e_eng, &e_exp) in energy.iter().zip(experience.iter()) {
            let eng_inc = if eng > e_eng { 0 } else { e_eng - eng + 1 };
            train += eng_inc;
            eng += eng_inc - e_eng;
            let exp_inc = if exp > e_exp { 0 } else { e_exp - exp + 1 };
            train += exp_inc;
            exp += exp_inc + e_exp;
        }
        train
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_number_of_hours() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        );
        assert_eq!(Solution::min_number_of_hours(2, 4, vec![1], vec![1]), 0);
    }
}
