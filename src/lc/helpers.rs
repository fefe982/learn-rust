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

pub enum Any {
    Str(&'static str),
    I32(i32),
    Vec(Vec<Any>),
}

impl From<&'static str> for Any {
    fn from(x: &'static str) -> Self {
        Any::Str(x)
    }
}
impl From<i32> for Any {
    fn from(x: i32) -> Self {
        Any::I32(x)
    }
}
impl From<Vec<Any>> for Any {
    fn from(x: Vec<Any>) -> Self {
        Any::Vec(x)
    }
}

#[macro_export]
macro_rules! vec_any {
    ($($x:tt),*$(,)?) => (vec![$(any_cast!($x)),*]);
}

#[macro_export]
macro_rules! any_cast {
    ([$($x:tt),*$(,)?]) => (vec![$(any_cast!($x)),*]);
    ($x:expr) => ($crate::lc::helpers::Any::from($x));
}
