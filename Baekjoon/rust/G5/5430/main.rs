use core::num;
// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::cmp::{max, min};
use std::collections::VecDeque;

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn solve(funcs: String, n: usize, array: String) -> Result<Vec<usize>, String> {
    let array_len = array.len();
    let mut deque: VecDeque<usize> = VecDeque::new();

    if n != 0 {
        array[1..(array_len - 1)]
            .split(',')
            .into_iter()
            .for_each(|num_str| deque.push_back(num_str.parse::<usize>().unwrap()));
    }

    let mut flag = true;
    for c in funcs.chars().into_iter() {
        match c {
            'R' => flag = !flag,
            'D' => {
                let result = if flag {
                    deque.pop_front()
                } else {
                    deque.pop_back()
                };
                match result {
                    None => return Err("error".to_string()),
                    _ => continue,
                };
            }
            _ => panic!("Invalid input"),
        };
    }

    let mut ret: Vec<usize> = vec![];
    deque.into_iter().for_each(|num| ret.push(num));

    if !flag {
        ret.reverse();
    }
    Ok(ret)
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let t: usize = line.next();
    for _ in 0..t {
        let funcs: &str = line.next_str();
        let n: usize = line.next();
        let arr_str = line.next_str();

        match solve(funcs.to_string(), n, arr_str.to_string()) {
            Ok(array) => {
                write!(buf_write, "[").ok();
                array.iter().enumerate().for_each(|(idx, num)| {
                    if idx == array.len() - 1 {
                        write!(buf_write, "{}", num).ok();
                    } else {
                        write!(buf_write, "{},", num).ok();
                    }
                });
                writeln!(buf_write, "]").ok()
            }
            Err(error) => writeln!(buf_write, "{}", error).ok(),
        };
    }

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
