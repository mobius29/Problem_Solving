use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

fn read() -> Tokenizer<SplitAsciiWhitespace<'static>> {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let str: &'static str = Box::leak(buf.into_boxed_str());

    Tokenizer::new(str, |s| s.split_ascii_whitespace())
}

const ACCEPT_STATES: [usize; 3] = [2, 6, 7];
const DEAD_STATE: usize = 9999;

const AUTOMATA: [[usize; 2]; 9] = [
    [1, 3],          // 0
    [DEAD_STATE, 2], // 1
    [1, 3],          // 2
    [4, DEAD_STATE], // 3
    [5, DEAD_STATE], // 4
    [5, 6],          // 5
    [1, 7],          // 6
    [8, 7],          // 7
    [5, 2],          // 8
];

fn solve(line: &str) -> bool {
    let mut cur_state: usize = 0;

    for c in line.chars() {
        let input = match c {
            '0' => 0,
            '1' => 1,
            _ => panic!("Invalid input"),
        };

        cur_state = AUTOMATA[cur_state][input];

        if cur_state == DEAD_STATE {
            return false;
        }
    }

    ACCEPT_STATES.contains(&cur_state)
}

fn main() {
    let line = &mut read();

    let t: usize = line.next();

    let mut results: Vec<bool> = Vec::new();
    for _ in 0..t {
        results.push(solve(line.next_str()));
    }

    // write
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());

    results.iter().for_each(|result| {
        let answer = match *result {
            true => "YES",
            false => "NO",
        };
        writeln!(buf_write, "{}", answer).ok();
    });

    buf_write.flush().unwrap();
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
