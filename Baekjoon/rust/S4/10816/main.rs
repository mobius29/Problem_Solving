use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::cmp::Ord;

fn find_equal_upperbound<T>(vector: &Vec<T>, key: &T) -> Option<usize>
where
    T: Ord,
{
    let mut left: i32 = 0;
    let mut right: i32 = vector.len() as i32 - 1;
    let mut return_value: Option<usize> = None;

    while left <= right {
        let m = (left + right) / 2;
        let idx: usize = m as usize;

        if vector[idx] < *key {
            left = m + 1;
        } else if vector[idx] > *key {
            right = m - 1;
        } else {
            return_value = Some(idx);
            left = m + 1;
        }
    }

    return_value
}

fn find_equal_lowerbound<T>(vector: &Vec<T>, key: &T) -> Option<usize>
where
    T: Ord,
{
    let mut left: i32 = 0;
    let mut right: i32 = vector.len() as i32 - 1;
    let mut return_value: Option<usize> = None;

    while left <= right {
        let m = (left + right) / 2;
        let idx = m as usize;

        if vector[idx] < *key {
            left = m + 1;
        } else if vector[idx] > *key {
            right = m - 1;
        } else {
            return_value = Some(idx);
            right = m - 1;
        }
    }

    return_value
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let n: usize = line.next();
    let mut cards: Vec<i32> = line.next_iter().take(n).collect();
    let m: usize = line.next();
    let find_numbers: Vec<i32> = line.next_iter().take(m).collect();

    let mut answers: Vec<usize> = Vec::new();

    cards.sort();

    for find_number in find_numbers.iter() {
        match find_equal_lowerbound(&cards, find_number) {
            None => answers.push(0),
            Some(lowerbound) => {
                let upperbound = find_equal_upperbound(&cards, find_number).unwrap();
                answers.push((upperbound - lowerbound + 1) as usize);
            }
        }
    }

    for answer in answers {
        write!(buf_write, "{} ", answer).ok();
    }
    // flush buffer
    buf_write.flush().unwrap();
}

fn read() -> Tokenizer<SplitAsciiWhitespace<'static>> {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let str: &'static str = Box::leak(buf.into_boxed_str());

    Tokenizer::new(str, |s| s.split_ascii_whitespace())
}

// Define Errors For Input
pub enum InputError<'t> {
    InputExhaust,
    ParseError(&'t str),
}

impl<'t> fmt::Debug for InputError<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::InputExhaust => f.debug_struct("InputExhaust").finish(),
            InputError::ParseError(s) => f.debug_struct("ParseError").field("str", s).finish(),
        }
    }
}

// Implements macros for parse string to certain type
pub trait Atom: Sized {
    fn parse_from(s: &str) -> Result<Self, InputError>;
}

trait IterParse: Sized {
    fn parse_from<'s, 't: 's, It>(it: &'s mut It) -> Result<Self, InputError<'t>>
    where
        It: Iterator<Item = &'t str>;
}

macro_rules! impl_trait_for_fromstr {
    ($($t:ty) *) => { $(
        impl Atom for $t {
            fn parse_from(s: &str) -> Result<Self, InputError> {
                s.parse().map_err(|_| InputError::ParseError(s))
            }
        }

        impl IterParse for $t {
            fn parse_from<'s, 't: 's, It>(it: &'s mut It) -> Result<Self, InputError<'t>> where It: Iterator<Item = &'t str> {
                it.next().map_or( Err(InputError::InputExhaust), <Self as Atom>::parse_from )
            }
        }
    )* };
}

impl_trait_for_fromstr!(bool char String);
impl_trait_for_fromstr!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);

macro_rules! impl_iterparse_for_tuple {
    ($($t:ident) *) => {
        impl<$($t),*> IterParse for ($($t),*) where $($t: IterParse),* {
            fn parse_from<'s, 't: 's, It>(it: &'s mut It) -> Result<Self, InputError<'t>> where It: Iterator<Item = &'t str> {
                Ok(( $($t::parse_from(it)?),* ))
            }
        }
    };
}

impl_iterparse_for_tuple!(A B);
impl_iterparse_for_tuple!(A B C);
impl_iterparse_for_tuple!(A B C D);

// Implements Tokenizer for split string line to variables
struct Tokenizer<It> {
    it: It,
}

impl<'arg, 'str: 'arg, It> Tokenizer<It> {
    pub fn new(s: &'str str, split: impl FnOnce(&'arg str) -> It) -> Self {
        Self { it: split(s) }
    }
}

impl<'t, It> Tokenizer<It>
where
    It: Iterator<Item = &'t str>,
{
    pub fn next<T: IterParse>(&mut self) -> T {
        T::parse_from(&mut self.it).unwrap()
    }

    pub fn next_str(&mut self) -> &'t str {
        self.it.next().unwrap()
    }

    pub fn next_ok<T: IterParse>(&mut self) -> Result<T, InputError<'t>> {
        T::parse_from(&mut self.it)
    }

    pub fn next_iter<T: IterParse>(&mut self) -> impl Iterator<Item = T> + '_ {
        std::iter::repeat_with(move || self.next_ok().ok()).map_while(|x| x)
    }
}
