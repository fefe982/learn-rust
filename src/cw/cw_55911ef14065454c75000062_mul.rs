// https://www.codewars.com/kata/55911ef14065454c75000062
// Multiplying numbers as strings
fn multiply(a: &str, b: &str) -> String {
    let a = a.as_bytes();
    let la = a.len();
    let b = b.as_bytes();
    let lb = b.len();
    let mut m = vec![0; a.len() + b.len() - 1];
    for ia in 0..a.len() {
        for ib in 0..b.len() {
            m[ia + ib] += ((a[la - ia - 1] - b'0') * (b[lb - ib - 1] - b'0')) as u32;
        }
    }
    let mut r = vec![];
    let mut carry = 0;
    for d in m {
        let s = d + carry;
        r.push(((s % 10) as u8 + b'0') as char);
        carry = s / 10;
    }
    while carry > 0 {
        r.push(((carry % 10) as u8 + b'0') as char);
        carry /= 10;
    }
    let mut iend = r.len() - 1;
    while iend > 0 && r[iend] == '0' {
        iend -= 1;
    }
    r[0..=iend].iter().rev().collect()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::multiply;

    fn do_test(a: &str, b: &str, expected: &str) {
        let actual = multiply(&a, &b);
        assert_eq!(actual, expected,
               "\n\nMultiplying a*b with\na = {a}\nb = {b}\nshould return: {expected}\ninstead got: {actual}");
    }

    #[test]
    fn simple_cases() {
        //        input       expected
        do_test("2", "3", "6");
        do_test("30", "69", "2070");
        do_test("11", "85", "935");
    }

    #[test]
    fn edge_cases() {
        do_test("2", "0", "0");
        do_test("0", "30", "0");
        do_test("0000001", "3", "3");
        do_test("1009", "03", "3027");
    }

    #[test]
    fn big_numbers() {
        do_test("98765", "56894", "5619135910");
        do_test(
            "9007199254740991",
            "9007199254740991",
            "81129638414606663681390495662081",
        );
        do_test(
            "1020303004875647366210",
            "2774537626200857473632627613",
            "2830869077153280552556547081187254342445169156730",
        );
        do_test(
            "58608473622772837728372827",
            "7586374672263726736374",
            "444625839871840560024489175424316205566214109298",
        );
    }
}
