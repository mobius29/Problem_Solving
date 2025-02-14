// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::cmp::{max, min};
use std::collections::VecDeque;

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn bfs(map: &Vec<Vec<usize>>, start: (usize, usize), size: (usize, usize)) -> Vec<Vec<i32>> {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);

    let mut visited: Vec<Vec<i32>> = vec![vec![-1; size.1]; size.0];
    visited[start.0][start.1] = 0;

    while !queue.is_empty() {
        let front = queue.pop_front().unwrap();
        let (x, y) = front;

        for (dx, dy) in ADJACENT {
            let next_x = x as i32 + dx;
            let next_y = y as i32 + dy;

            let next_x = if next_x >= 0 {
                Some(next_x as usize)
            } else {
                None
            };
            let next_y = if next_y >= 0 {
                Some(next_y as usize)
            } else {
                None
            };

            let (next_x, next_y) = match (next_x, next_y) {
                (Some(x), Some(y)) if x < size.0 && y < size.1 => (x, y),
                (_, _) => continue,
            };

            if visited[next_x][next_y] != -1 {
                continue;
            }

            if map[next_x][next_y] == 0 {
                visited[next_x][next_y] = 0;
            }

            if map[next_x][next_y] == 1 {
                visited[next_x][next_y] = visited[x][y] + 1;
                queue.push_back((next_x, next_y));
            }
        }
    }

    visited
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let (n, m): (usize, usize) = line.next();
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut start = (0, 0);
    for i in 0..n {
        let mut row: Vec<usize> = Vec::new();
        for j in 0..m {
            let item = line.next();
            if item == 2 {
                start = (i, j);
            }
            row.push(item);
        }
        map.push(row);
    }

    let answer = bfs(&map, start, (n, m));
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 0 {
                write!(buf_write, "0 ").ok();
            } else {
                write!(buf_write, "{} ", answer[i][j]).ok();
            }
        }

        writeln!(buf_write).ok();
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
