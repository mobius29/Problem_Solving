use std::cmp::max;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

const INF_I32: i32 = 0x3F3F3F;
const INF_USIZE: usize = 0x3F3F3F;

fn find_shortest_paths(paths: &Vec<Vec<(usize, usize)>>, start: usize, n: usize) -> Vec<usize> {
    let mut shortest_path = vec![INF_USIZE; n + 1];
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((start, 0));
    shortest_path[start] = 0;

    while let Some((cur, cur_cost)) = priority_queue.pop() {
        paths[cur].iter().for_each(|(next, cost)| {
            let next = *next;
            let next_cost = cur_cost + cost;
            if next_cost < shortest_path[next] {
                shortest_path[next] = next_cost;
                priority_queue.push((next, next_cost));
            }
        });
    }

    shortest_path
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let (n, m, x): (usize, usize, usize) = line.next();
    let mut paths: Vec<Vec<(usize, usize)>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b, t): (usize, usize, usize) = line.next();
        paths[a].push((b, t));
    }

    let mut shortest_paths = vec![vec![]; n + 1];
    for i in 1..=n {
        shortest_paths[i] = find_shortest_paths(&paths, i, n);
    }

    let mut shortest_costs = vec![];
    for i in 1..=n {
        shortest_costs.push(shortest_paths[i][x] + shortest_paths[x][i]);
    }

    let max_value = shortest_costs
        .into_iter()
        .enumerate()
        .fold((0, 0), |acc, cur| if cur.1 > acc.1 { cur } else { acc })
        .1;

    writeln!(buf_write, "{}", max_value).ok();
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
