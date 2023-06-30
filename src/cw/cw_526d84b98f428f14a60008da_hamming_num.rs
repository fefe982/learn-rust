// https://www.codewars.com/kata/526d84b98f428f14a60008da
// Hamming Numbers
pub fn hamming(n: usize) -> u64 {
    let mut q = std::collections::BinaryHeap::new();
    q.push(std::cmp::Reverse(1));
    for _ in 0..n - 1 {
        let i: u64 = q.pop().unwrap().0;
        if let Some(i5) = i.checked_mul(5) {
            q.push(std::cmp::Reverse(i5));
        }
        if i % 5 != 0 {
            if let Some(i3) = i.checked_mul(3) {
                q.push(std::cmp::Reverse(i3));
            }
            if i % 3 != 0 {
                if let Some(i2) = i.checked_mul(2) {
                    q.push(std::cmp::Reverse(i2));
                }
            }
        }
    }
    q.pop().unwrap().0
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::hamming;

    #[test]
    fn sample_tests() {
        assert_eq!(hamming(1), 1);
        assert_eq!(hamming(2), 2);
        assert_eq!(hamming(3), 3);
        assert_eq!(hamming(4), 4);
        assert_eq!(hamming(5), 5);
        assert_eq!(hamming(6), 6);
        assert_eq!(hamming(7), 8);
        assert_eq!(hamming(8), 9);
        assert_eq!(hamming(9), 10);
        assert_eq!(hamming(10), 12);
        assert_eq!(hamming(11), 15);
        assert_eq!(hamming(12), 16);
        assert_eq!(hamming(13), 18);
        assert_eq!(hamming(14), 20);
        assert_eq!(hamming(15), 24);
        assert_eq!(hamming(16), 25);
        assert_eq!(hamming(17), 27);
        assert_eq!(hamming(18), 30);
        assert_eq!(hamming(19), 32);
    }
}
