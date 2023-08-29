// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

const DIRECTION: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn bfs(maze: &Vec<Vec<bool>>, size: (usize, usize), dest: (usize, usize)) -> Option<usize> {
    let mut deque: VecDeque<(Point, usize)> = VecDeque::new();
    deque.push_back((Point { x: 0, y: 0 }, 1));

    let mut visited: Vec<Vec<bool>> = vec![vec![false; size.1]; size.0];
    visited[0][0] = true;

    while !deque.is_empty() {
        let (Point { x, y }, count) = deque.pop_front().unwrap();
        if x == dest.0 && y == dest.1 {
            return Some(count);
        }

        let count = count + 1;
        for (dx, dy) in DIRECTION {
            let (x, y) = match (x as i32 + dx, y as i32 + dy) {
                (x, y) if x < 0 || y < 0 => continue,
                (x, y) if x as usize >= size.0 || y as usize >= size.1 => continue,
                (x, y) => (x as usize, y as usize),
            };

            if !visited[x][y] && maze[x][y] {
                deque.push_back((Point { x, y }, count));
            }
            visited[x][y] = true;
        }
    }

    None
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());

    let line = &mut read();
    let (n, m): (usize, usize) = line.next();
    let mut maze: Vec<Vec<bool>> = vec![vec![false; m]; n];
    for i in 0..n {
        let str = line.next_str();
        str.chars().enumerate().for_each(|(j, c)| {
            if c == '1' {
                maze[i][j] = true;
            }
        });
    }

    let answer = bfs(&maze, (n, m), (n - 1, m - 1)).unwrap();

    writeln!(buf_write, "{}", answer).ok();
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
