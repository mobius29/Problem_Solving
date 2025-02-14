use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::cmp::{max, min};
use std::collections::VecDeque;

type Vec2<T> = Vec<Vec<T>>;
type Vec3<T> = Vec<Vec<Vec<T>>>;

type Tuple2<T> = (T, T);
type Tuple3<T> = (T, T, T);

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const ADJACENT_3D: [Tuple3<i32>; 6] = [
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
];

const INF_USIZE: usize = 0x3F3F3F;

const TETROMINOES: [[Tuple2<i32>; 4]; 19] = [
    [(0, 0), (0, 1), (0, 2), (0, 3)], // 4
    [(0, 0), (0, 1), (0, 2), (1, 0)], // 3, 1
    [(0, 0), (0, 1), (0, 2), (1, 2)],
    [(0, 0), (0, 1), (1, 0), (1, 1)],  // 2, 2
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(0, 0), (1, 0), (0, 1), (-1, 1)],
    [(0, 0), (0, 1), (-1, 1), (-1, 2)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
    [(0, 0), (1, 0), (2, 0), (0, -1)], // 2, 1, 1
    [(0, 0), (1, 0), (2, 0), (0, 1)],
    [(0, 0), (0, 1), (0, 2), (-1, 0)], // 1, 3
    [(0, 0), (0, 1), (0, 2), (-1, 2)],
    [(0, 0), (0, 1), (0, 2), (-1, 1)],
    [(0, 0), (0, 1), (0, 2), (1, 1)],
    [(0, 0), (1, 0), (2, 0), (1, -1)], // 1, 2, 1
    [(0, 0), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (2, 0), (2, -1)], // 1, 1, 2
    [(0, 0), (1, 0), (2, 0), (2, 1)],
    [(0, 0), (1, 0), (2, 0), (3, 0)], // 1, 1, 1, 1
];

struct Board<T> {
    size: Tuple2<usize>,
    values: Vec2<T>,
}

impl Board<usize> {
    pub fn new(size: Tuple2<usize>, values: Vec2<usize>) -> Self {
        Board { size, values }
    }

    fn is_in_range(&self, point: (i32, i32)) -> bool {
        let (h, w) = self.size;
        let (x, y) = point;

        let x_is_valid = 0 <= x && (x as usize) < h;
        let y_is_valid = 0 <= y && (y as usize) < w;

        x_is_valid && y_is_valid
    }

    fn move_point(&self, point: (usize, usize), d: &Tuple2<i32>) -> Result<Tuple2<usize>, ()> {
        let ((x, y), (dx, dy)) = (point, d);
        let (next_x, next_y) = ((x as i32) + dx, (y as i32) + dy);

        if self.is_in_range((next_x, next_y)) {
            Ok((next_x as usize, next_y as usize))
        } else {
            Err(())
        }
    }

    fn find_max(&self, point: (usize, usize)) -> usize {
        let mut ret = 0;
        for d_list in TETROMINOES.iter() {
            let mut value = 0;
            for d in d_list.iter() {
                match self.move_point(point, d) {
                    Ok((next_x, next_y)) => {
                        value += self.values[next_x][next_y];
                    }
                    Err(_) => {
                        value = 0;
                        break;
                    }
                }
            }

            ret = max(ret, value);
        }

        ret
    }

    pub fn solve(&self) -> usize {
        let (h, w) = self.size;
        let mut ret = 0;

        for i in 0..h {
            for j in 0..w {
                ret = max(ret, self.find_max((i, j)));
            }
        }
        ret
    }
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let (h, w): (usize, usize) = line.next();
    let mut values: Vec2<usize> = vec![];
    for _ in 0..h {
        let mut row: Vec<usize> = vec![];
        for _ in 0..w {
            let item: usize = line.next();
            row.push(item);
        }
        values.push(row);
    }

    let board: Board<usize> = Board::new((h, w), values);
    let answer = board.solve();

    writeln!(buf_write, "{}", answer).ok();

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
