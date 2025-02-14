use std::cmp::max;
use std::collections::VecDeque;
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

const D: [(isize, isize, isize); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

struct Board {
    size: (usize, usize, usize),
    tomatoes: Vec<Vec<Vec<i8>>>,
}

impl Board {
    pub fn new(size: (usize, usize, usize), tomatoes: Vec<Vec<Vec<i8>>>) -> Self {
        Self { size, tomatoes }
    }

    pub fn is_in_range(
        &self,
        point: (usize, usize, usize),
        d: &(isize, isize, isize),
    ) -> Option<(usize, usize, usize)> {
        let (size_h, size_n, size_m) = self.size;
        let (h, n, m) = point;
        let (dh, dn, dm) = d;

        let h = h as isize + dh;
        let n = n as isize + dn;
        let m = m as isize + dm;

        if h < 0 || n < 0 || m < 0 {
            return None;
        }

        let h = h as usize;
        let n = n as usize;
        let m = m as usize;

        if h >= size_h || n >= size_n || m >= size_m {
            return None;
        }

        Some((h, n, m))
    }

    pub fn is_all_tomatoes_ripened(&self) -> bool {
        let mut flag = true;
        self.tomatoes.iter().for_each(|area| {
            area.iter().for_each(|row| {
                row.iter().for_each(|item| {
                    if *item == 0 {
                        flag = false;
                    }
                })
            })
        });

        flag
    }
}

fn push_initial_point(board: &Board) -> VecDeque<((usize, usize, usize), i32)> {
    let mut queue = VecDeque::new();
    for i in 0..board.size.0 {
        for j in 0..board.size.1 {
            for k in 0..board.size.2 {
                if board.tomatoes[i][j][k] == 1 {
                    queue.push_back(((i, j, k), 0));
                }
            }
        }
    }

    queue
}

fn solve(board: &Board) -> i32 {
    let mut new_board = Board::new(board.size, board.tomatoes.clone());
    let mut queue = push_initial_point(&new_board);

    let mut answer = 0;
    while let Some((cur, mut count)) = queue.pop_front() {
        answer = count;
        count += 1;

        D.iter().for_each(|d| {
            if let Some(next) = board.is_in_range(cur, d) {
                let (h, n, m) = next;

                if new_board.tomatoes[h][n][m] == 0 {
                    new_board.tomatoes[h][n][m] = 1;
                    queue.push_back((next, count));
                }
            }
        });
    }

    if !new_board.is_all_tomatoes_ripened() {
        answer = -1;
    }

    answer
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let (m, n, h): (usize, usize, usize) = line.next();
    let mut tomatoes: Vec<Vec<Vec<i8>>> = vec![vec![vec![0; m]; n]; h];
    for i in 0..h {
        for j in 0..n {
            for k in 0..m {
                tomatoes[i][j][k] = line.next();
            }
        }
    }

    let board = Board::new((h, n, m), tomatoes);

    if board.is_all_tomatoes_ripened() {
        writeln!(buf_write, "0").ok();
    } else {
        writeln!(buf_write, "{}", solve(&board)).ok();
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