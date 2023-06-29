// https://www.codewars.com/kata/5a24a35a837545ab04001614
// Interlaced Spiral Cipher
pub mod isc {
    pub fn encode(s: &str) -> String {
        let l = s.len();
        let mut n = (l as f64).sqrt() as usize;
        if n * n < l {
            n += 1;
        }
        let mut v = vec![' '; n * n];
        let mut i = 0;
        let s = s.as_bytes();
        'lbl: for il in 0..n / 2 {
            for is in 0..n - il * 2 - 1 {
                for ic in 0..4 {
                    let (iy, ix) = match ic {
                        0 => (il, il + is),
                        1 => (il + is, n - il - 1),
                        2 => (n - il - 1, n - il - 1 - is),
                        3 => (n - il - 1 - is, il),
                        _ => unreachable!(),
                    };
                    v[iy * n + ix] = s[i] as char;
                    i += 1;
                    if i >= l {
                        break 'lbl;
                    }
                }
            }
        }
        if i < l {
            v[l / 2] = s[i] as char;
        }
        v.into_iter().collect()
    }

    pub fn decode(s: &str) -> String {
        let l = s.len();
        let mut n = (l as f64).sqrt() as usize;
        if n * n < l {
            n += 1;
        }
        let mut v = vec![' '; n * n];
        let mut i = 0;
        let s = s.as_bytes();
        for il in 0..n / 2 {
            for is in 0..n - il * 2 - 1 {
                for ic in 0..4 {
                    let (iy, ix) = match ic {
                        0 => (il, il + is),
                        1 => (il + is, n - il - 1),
                        2 => (n - il - 1, n - il - 1 - is),
                        3 => (n - il - 1 - is, il),
                        _ => unreachable!(),
                    };
                    v[i] = s[iy * n + ix] as char;
                    i += 1;
                }
            }
        }
        if i < l {
            v[i] = s[l / 2] as char;
        }
        v.into_iter().collect::<String>().trim_end().to_owned()
    }
}
#[cfg(test)]
mod example_tests {
    use super::*;

    fn run_test(s1: &str, s2: &str) {
        assert_eq!(&isc::encode(s1), s2);
        assert_eq!(&isc::decode(s2), s1);
    }

    #[test]
    fn example_test_1() {
        let example_1a = "Romani ite domum";
        let example_1b = "Rntodomiimuea  m";
        run_test(example_1a, example_1b);
    }

    #[test]
    fn example_test_2() {
        let example_2a = "Sic transit gloria mundi";
        let example_2b = "Stsgiriuar i ninmd l otac";
        run_test(example_2a, example_2b);
    }
}
