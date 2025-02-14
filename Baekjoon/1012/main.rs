use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::collections::VecDeque;

static NEXT: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Board {
    width: usize,
    height: usize,
}

fn is_valid_range(current: usize, d: i32, min: usize, max: usize) -> Option<usize> {
    match current as i32 + d {
        value if value < min as i32 || value >= max as i32 => None,
        value => Some(value as usize),
    }
}

fn bfs(array: &Vec<Vec<bool>>, checked: &mut Vec<Vec<bool>>, start: Point, size: Board) {
    let mut q: VecDeque<Point> = VecDeque::new();
    q.push_back(start);

    while !q.is_empty() {
        let Point { x, y } = q.pop_front().unwrap();

        NEXT.iter().for_each(|(dx, dy)| {
            let next_x = match is_valid_range(x, *dx, 0, size.width) {
                Some(v) => v,
                None => return,
            };
            let next_y = match is_valid_range(y, *dy, 0, size.height) {
                Some(v) => v,
                None => return,
            };

            if array[next_x][next_y] && !checked[next_x][next_y] {
                checked[next_x][next_y] = true;
                q.push_back(Point {
                    x: next_x,
                    y: next_y,
                });
            }
        });
    }
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let t: usize = line.next();
    for _ in 0..t {
        let (m, n, k): (usize, usize, usize) = line.next();
        let board_size = Board {
            width: m,
            height: n,
        };
        let mut cabbage_field: Vec<Vec<bool>> = vec![vec![false; n]; m];

        for _ in 0..k {
            let (x, y): (usize, usize) = line.next();
            cabbage_field[x][y] = true;
        }

        let mut checked: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let mut answer = 0;
        for i in 0..m {
            for j in 0..n {
                if cabbage_field[i][j] && !checked[i][j] {
                    let start_point = Point { x: i, y: j };
                    bfs(&cabbage_field, &mut checked, start_point, board_size);
                    answer += 1;
                }
            }
        }

        writeln!(buf_write, "{}", answer).ok();
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
