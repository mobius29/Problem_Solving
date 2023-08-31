// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::cmp::{max, min};
use std::collections::VecDeque;

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Board {
    m: usize,
    n: usize,
}

fn propagation(tomatoes: &mut Vec<Vec<i32>>, board: Board) -> usize {
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    for i in 0..board.n {
        for j in 0..board.m {
            if tomatoes[i][j] == 1 {
                let point = Point {
                    x: i.try_into().unwrap(),
                    y: j.try_into().unwrap(),
                };
                queue.push_back((point, 0));
            }
        }
    }

    let mut answer = 0;
    while !queue.is_empty() {
        let (point, days) = queue.pop_front().unwrap();
        answer = max(answer, days);

        let Point { x, y } = point;
        ADJACENT.iter().for_each(|(dx, dy)| {
            let x = x + dx;
            let y = y + dy;

            if x < 0 || y < 0 || x >= board.n as i32 || y >= board.m as i32 {
                return;
            }

            let x = x as usize;
            let y = y as usize;

            if tomatoes[x][y] != 0 {
                return;
            }
            tomatoes[x][y] = 1;

            let x = x as i32;
            let y = y as i32;
            queue.push_back((Point { x, y }, days + 1));
        });
    }

    answer
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let (m, n): (usize, usize) = line.next();
    let mut tomatoes: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            let status: i32 = line.next();
            tomatoes[i][j] = status;
        }
    }

    let answer = propagation(&mut tomatoes, Board { m, n });

    let mut flag = false;
    for i in 0..n {
        for j in 0..m {
            if tomatoes[i][j] == 0 {
                flag = true;
            }
        }
    }

    if flag {
        writeln!(buf_write, "-1").ok();
    } else {
        writeln!(buf_write, "{}", answer).ok();
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
