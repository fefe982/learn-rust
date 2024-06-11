pub fn make_string_vec<const N: usize>(arr_in: [&str; N]) -> Vec<String> {
    arr_in.into_iter().map(|x| String::from(x)).collect()
}

#[macro_export]
macro_rules! vec_str {
    ($($x:expr),*$(,)?) => (vec![$(String::from($x)),*]);
}
#[macro_export]
macro_rules! vec_vec {
    ($([$($x:expr),*$(,)?]),*$(,)?) => (vec![$(vec![$($x),*]),*]);
}
#[macro_export]
macro_rules! vec_vec_str {
    ($($x:tt),*$(,)?) => (vec![$(vec_str!$x),*]);
}
#[macro_export]
macro_rules! vec_chr {
    ($($x:expr),*$(,)?) => (vec![$($x.chars().next().unwrap()),*]);
}
#[macro_export]
macro_rules! vec_vec_chr {
    ($($x:tt),*$(,)?) => (vec![$(vec_chr!$x),*]);
}
