// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::cmp::{max, min};

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn dfs(map: &Vec<Vec<bool>>, current: Point, checked: &mut Vec<Vec<bool>>, size: usize) -> usize {
    let mut ret = 1;
    let Point { x, y } = current;

    for (dx, dy) in ADJACENT {
        let x = x as i32 + dx;
        let y = y as i32 + dy;
        let (x, y) = match (x, y) {
            (x, y) if x < 0 || y < 0 => continue,
            (x, y) if x as usize >= size || y as usize >= size => continue,
            (x, y) => (x as usize, y as usize),
        };

        let point = Point { x, y };

        if map[x][y] && !checked[x][y] {
            checked[x][y] = true;
            ret += dfs(map, point, checked, size);
        }
    }

    ret
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let n: usize = line.next();
    let mut map: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for i in 0..n {
        let str = line.next_str();
        for (j, c) in str.chars().enumerate().into_iter() {
            if c == '1' {
                map[i][j] = true;
            }
        }
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut answer: Vec<usize> = vec![];
    for x in 0..n {
        for y in 0..n {
            if map[x][y] && !visited[x][y] {
                visited[x][y] = true;
                let count = dfs(&map, Point { x, y }, &mut visited, n);
                answer.push(count);
            }
        }
    }

    answer.sort();

    writeln!(buf_write, "{}", answer.len()).ok();
    for count in answer {
        writeln!(buf_write, "{}", count).ok();
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
