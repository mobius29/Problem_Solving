// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

fn read() -> Tokenizer<SplitAsciiWhitespace<'static>> {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let str: &'static str = Box::leak(buf.into_boxed_str());

    Tokenizer::new(str, |s| s.split_ascii_whitespace())
}

fn count_the_number_of_squares_to_recolor(
    board: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
) -> usize {
    let ideal_board_from_white: Vec<&str> = vec![
        "WBWBWBWB", "BWBWBWBW", "WBWBWBWB", "BWBWBWBW", "WBWBWBWB", "BWBWBWBW", "WBWBWBWB",
        "BWBWBWBW",
    ];
    let ideal_board_from_black: Vec<&str> = vec![
        "BWBWBWBW", "WBWBWBWB", "BWBWBWBW", "WBWBWBWB", "BWBWBWBW", "WBWBWBWB", "BWBWBWBW",
        "WBWBWBWB",
    ];

    let mut count_from_white = 0;
    let mut count_from_black = 0;

    for i in start_x..(start_x + 8) {
        let board_x = i - start_x;
        let ideal_line_from_black: Vec<char> = ideal_board_from_black[board_x].chars().collect();
        let ideal_line_from_white: Vec<char> = ideal_board_from_white[board_x].chars().collect();

        for j in start_y..(start_y + 8) {
            let board_y = j - start_y;
            if board[i][j] != ideal_line_from_black[board_y] {
                count_from_black += 1;
            }

            if board[i][j] != ideal_line_from_white[board_y] {
                count_from_white += 1;
            }
        }
    }

    if count_from_white < count_from_black { count_from_white } else { count_from_black }
}

fn main() {
    // read
    let line = &mut read();

    // read all line and tokenize by ascii white space
    // write
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());

    let (n, m): (usize, usize) = line.next();
    let mut board: Vec<Vec<char>> = Vec::new();

    for _i in 0..n {
        let str: &str = line.next_str();
        board.push(str.chars().collect());
    }

    let mut answer: usize = 64;

    for i in 0..=(n - 8) {
        for j in 0..=(m - 8) {
            let count = count_the_number_of_squares_to_recolor(&board, i, j);
            if count < answer {
                answer = count;
            }
        }
    }

    writeln!(buf_write, "{}", answer).ok();

    // flush buffer
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
