use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

const INF_I32: i32 = 0x3F3F3F3F;
const INF_USIZE: usize = 0x3F3F3F3F;

type Vec2<T> = Vec<Vec<T>>;
type Pair<T> = (T, T);
type Tuple<T> = (T, T, T);

fn input() -> (usize, Vec<Vec<usize>>, usize, usize) {
    let line = &mut read();

    let (n, m): Pair<usize> = line.next();
    let mut arr = vec![vec![INF_USIZE; n + 1]; n + 1];
    for _ in 0..m {
        let (a, b, c): Tuple<usize> = line.next();
        arr[a][b] = min(arr[a][b], c);
    }
    let (src, dest): Pair<usize> = line.next();

    (n, arr, src, dest)
}

fn output(answer: usize) {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());

    writeln!(buf_write, "{}", answer).ok();

    buf_write.flush().unwrap();
}

fn find_shortest_path(distances: &Vec<usize>, visited: &Vec<bool>, n: usize) -> Option<usize> {
    let mut min_value = INF_USIZE;
    let mut min_idx = None;

    for i in 1..=n {
        if !visited[i] && distances[i] < min_value {
            min_value = distances[i];
            min_idx = Some(i);
        }
    }

    min_idx
}

fn update_shortest_paths(
    paths: &Vec2<usize>,
    distances: &mut Vec<usize>,
    visited: &Vec<bool>,
    idx: usize,
    n: usize,
) {
    for i in 1..=n {
        if !visited[i] {
            distances[i] = min(distances[i], distances[idx] + paths[idx][i]);
        }
    }
}

fn solve(paths: &Vec2<usize>, src: usize, dest: usize, n: usize) -> usize {
    let mut distances = paths[src].clone();
    let mut visited = vec![false; n + 1];

    distances[src] = 0;
    visited[src] = true;

    while let Some(min_idx) = find_shortest_path(&distances, &visited, n) {
        visited[min_idx] = true;
        update_shortest_paths(paths, &mut distances, &visited, min_idx, n);
    }

    distances[dest]
}

fn main() {
    let (n, paths, src, dest) = input();
    let answer = solve(&paths, src, dest, n);
    output(answer);
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
