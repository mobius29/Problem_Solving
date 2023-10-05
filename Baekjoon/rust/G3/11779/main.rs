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

fn find_shortest_path(dist: &Vec<usize>, visited: &Vec<bool>, n: usize) -> Option<usize> {
    let mut min_dist = INF_USIZE;
    let mut min_idx = None;

    for i in 1..=n {
        if !visited[i] && dist[i] < min_dist {
            min_dist = dist[i];
            min_idx = Some(i);
        }
    }

    min_idx
}

fn update_shortest_path(
    path: &Vec2<usize>,
    dist: &mut Vec<usize>,
    visited: &Vec<bool>,
    min_path: &mut Vec<usize>,
    idx: usize,
    n: usize,
) {
    for i in 1..=n {
        let new_cost = dist[idx] + path[idx][i];
        if !visited[i] && new_cost < dist[i] {
            min_path[i] = idx;
            dist[i] = new_cost;
        }
    }
}

fn solve(paths: &Vec2<usize>, src: usize, dest: usize, n: usize) -> (usize, Vec<usize>) {
    let mut dist = paths[src].clone();
    dist[src] = 0;

    let mut visited = vec![false; n + 1];
    visited[src] = true;

    let mut min_path = vec![0; n + 1];
    for i in 1..=n {
        if dist[i] != 0 && dist[i] != INF_USIZE {
            min_path[i] = src;
        }
    }

    while let Some(min_idx) = find_shortest_path(&dist, &visited, n) {
        visited[min_idx] = true;
        update_shortest_path(paths, &mut dist, &visited, &mut min_path, min_idx, n);
    }

    let mut path = vec![];
    let mut i = dest;
    while i != 0 {
        path.push(i);
        i = min_path[i];
    }
    path.reverse();

    (dist[dest], path)
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let n: usize = line.next();
    let m: usize = line.next();
    let mut paths = vec![vec![INF_USIZE; n + 1]; n + 1];
    for _ in 0..m {
        let (a, b, c): Tuple<usize> = line.next();
        paths[a][b] = min(paths[a][b], c);
    }
    let (src, dest): Pair<usize> = line.next();

    let (cost, path) = solve(&paths, src, dest, n);

    writeln!(buf_write, "{}", cost).ok();
    writeln!(buf_write, "{}", path.len());
    for i in path.into_iter() {
        write!(buf_write, "{} ", i).ok();
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
