// https://www.codewars.com/kata/52ec24228a515e620b0005ef
// Explosive Sum
pub fn exp_sum(n: u64) -> u64 {
    let mut v = vec![vec![0u64; n as usize + 1]; n as usize + 1];
    for i in 1..=n as usize {
        v[i][i] = 1;
        let mut sum = 1;
        for j in (1..i).rev() {
            sum += v[i - j][j];
            v[i][j] = sum;
        }
    }
    v[n as usize][1]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sample_tests() {
        assert_eq!(exp_sum(1), 1);
        assert_eq!(exp_sum(2), 2);
        assert_eq!(exp_sum(3), 3);
        assert_eq!(exp_sum(4), 5);
        assert_eq!(exp_sum(5), 7);
        assert_eq!(exp_sum(10), 42);
    }
}
