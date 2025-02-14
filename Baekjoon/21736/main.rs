use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::cmp::{max, min};
use std::collections::VecDeque;

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Board<T>
where
    T: Clone,
{
    width: usize,
    height: usize,
    value: Vec<Vec<T>>,
}

impl<T> Board<T>
where
    T: Clone,
{
    fn new(width: usize, height: usize, initial_value: T) -> Self {
        Self {
            width,
            height,
            value: vec![vec![initial_value; width]; height],
        }
    }

    fn get_board_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn is_valid_move(&self, d: (i32, i32), max: (usize, usize)) -> Option<Point> {
        let x = self.x as i32 + d.0;
        let y = self.y as i32 + d.1;
        if x < 0 || y < 0 || x >= max.1 as i32 || y >= max.0 as i32 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        Some(Point { x, y })
    }
}

fn find_doyeon(board: &Board<char>) -> Option<Point> {
    let mut ret = None;
    let (width, height) = board.get_board_size();

    'row: for i in 0..height {
        for j in 0..width {
            if board.value[i][j] == 'I' {
                ret = Some(Point { x: i, y: j });
                break 'row;
            }
        }
    }

    ret
}

fn bfs(
    board: &Board<char>,
    queue: &mut VecDeque<Point>,
    visited: &mut Vec<Vec<bool>>,
    answer: &mut usize,
) {
    let current = match queue.pop_front() {
        Some(v) => v,
        None => return,
    };

    let (width, height) = board.get_board_size();

    ADJACENT
        .iter()
        .for_each(|d| match current.is_valid_move(*d, (width, height)) {
            Some(Point { x, y }) => {
                if !visited[x][y] {
                    visited[x][y] = true;
                    match board.value[x][y] {
                        'O' => queue.push_back(Point { x, y }),
                        'P' => {
                            queue.push_back(Point { x, y });
                            *answer += 1;
                        }
                        _ => return,
                    };
                }
            }
            None => return,
        });

    bfs(board, queue, visited, answer);
}

fn solve(board: &Board<char>) -> usize {
    let (width, height) = board.get_board_size();
    let start = find_doyeon(board).unwrap();

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(start.clone());

    let mut visited = vec![vec![false; width]; height];
    visited[start.x][start.y] = true;

    let mut answer = 0;

    bfs(board, &mut queue, &mut visited, &mut answer);
    answer
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let (height, width): (usize, usize) = line.next();
    let mut board = Board::new(width, height, 'O');
    for i in 0..board.height {
        let row = line.next_str();
        row.chars()
            .into_iter()
            .enumerate()
            .into_iter()
            .for_each(|(j, c)| board.value[i][j] = c);
    }

    let answer = solve(&board);
    if answer == 0 {
        writeln!(buf_write, "TT").ok();
    } else {
        writeln!(buf_write, "{}", answer).ok();
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
