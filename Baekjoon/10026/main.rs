use crate::InputError::ParseError;
use std::collections::VecDeque;
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

const D: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn move_point(point: (usize, usize), d: (isize, isize), n: usize) -> Option<(usize, usize)> {
    let next_x = point.0 as isize + d.0;
    let next_y = point.1 as isize + d.1;

    let next_x = if next_x < 0 {
        None
    } else {
        Some(next_x as usize)
    };
    let next_y = if next_y < 0 {
        None
    } else {
        Some(next_y as usize)
    };

    match (next_x, next_y) {
        (Some(x), Some(y)) if x < n && y < n => Some((x, y)),
        (_, _) => None,
    }
}

fn dfs_not_blindness(
    drawing: &Vec<Vec<char>>,
    cur: (usize, usize),
    is_visited: &mut Vec<Vec<bool>>,
    n: usize,
) {
    let (cur_x, cur_y) = cur;

    for d in D {
        if let Some(next) = move_point(cur, d, n) {
            let (next_x, next_y) = next;
            if !is_visited[next_x][next_y] && drawing[cur_x][cur_y] == drawing[next_x][next_y] {
                is_visited[next_x][next_y] = true;
                dfs_not_blindness(drawing, next, is_visited, n);
            }
        }
    }
}

fn dfs_blindness(
    drawing: &Vec<Vec<char>>,
    cur: (usize, usize),
    is_visited: &mut Vec<Vec<bool>>,
    n: usize,
) {
    let (cur_x, cur_y) = cur;

    for d in D {
        if let Some(next) = move_point(cur, d, n) {
            let (next_x, next_y) = next;
            if is_visited[next_x][next_y] {
                continue;
            }

            let is_current_blue = drawing[cur_x][cur_y] == 'B';
            let is_next_blue = drawing[next_x][next_y] == 'B';

            if (is_current_blue && is_next_blue) || (!is_current_blue && !is_next_blue) {
                is_visited[next_x][next_y] = true;
                dfs_blindness(drawing, next, is_visited, n);
            }
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let n: usize = line.next();
    let mut drawing: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let row = line.next_str();
        drawing.push(row.chars().collect::<Vec<char>>());
    }

    let mut not_red_green_color_blindness = 0;
    let mut is_visited = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if !is_visited[i][j] {
                not_red_green_color_blindness += 1;
                is_visited[i][j] = true;
                dfs_not_blindness(&drawing, (i, j), &mut is_visited, n);
            }
        }
    }

    let mut red_green_color_bilndness = 0;
    let mut is_visited = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if !is_visited[i][j] {
                red_green_color_bilndness += 1;
                is_visited[i][j] = true;
                dfs_blindness(&drawing, (i, j), &mut is_visited, n);
            }
        }
    }

    writeln!(
        buf_write,
        "{} {}",
        not_red_green_color_blindness, red_green_color_bilndness
    )
    .ok();

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
