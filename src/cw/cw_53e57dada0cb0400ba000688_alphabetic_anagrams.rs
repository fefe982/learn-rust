// https://www.codewars.com/kata/53e57dada0cb0400ba000688
// Alphabetic Anagrams
fn fact(n: usize, v: &mut Vec<u128>) -> u128 {
    while v.len() <= n {
        v.push(v.last().unwrap() * v.len() as u128);
    }
    v[n]
}
pub fn list_position(word: &str) -> u128 {
    let mut f = vec![1];
    let mut cnt = vec![0; 26];
    let mut min = 25;
    let mut sum = 1;
    let mut denorm = 1;
    for (i, c) in word.bytes().into_iter().rev().enumerate() {
        let c = (c - b'A') as usize;
        if c >= min {
            let fc = fact(i, &mut f) / denorm;
            for ic in min..c {
                if cnt[ic] > 0 {
                    sum += fc * cnt[ic] / (cnt[c] + 1);
                }
            }
        }
        cnt[c] += 1;
        min = min.min(c);
        denorm *= cnt[c] as u128;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::list_position;
    #[test]
    fn sample_tests() {
        let test_data = [
            ("A", 1),
            ("ABAB", 2),
            ("AAAB", 1),
            ("BAAA", 4),
            ("YMYM", 5),
            ("QUESTION", 24572),
            ("BOOKKEEPER", 10743),
            ("IMMUNOELECTROPHORETICALLY", 718393983731145698173),
        ];
        for (word, expected) in test_data {
            assert_eq!(list_position(word),
                       expected,
                       "\nYour result (left) did not match the expected output (right) for the input: \"{word}\"");
        }
    }
}
