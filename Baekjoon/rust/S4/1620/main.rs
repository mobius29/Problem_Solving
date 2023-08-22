use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

#[derive(Clone)]
struct Pokemon {
    id: usize,
    name: String,
}

fn binary_search(arr: &Vec<Pokemon>, key: String) -> Result<usize, String> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let m = (left + right) / 2;

        if arr[m].name == key {
            return Ok(arr[m].id);
        }

        if arr[m].name < key {
            left = m + 1;
        } else {
            match m.checked_sub(1) {
                Some(value) => right = value,
                None => break,
            };
        }
    }

    Err("Can't find key".to_string())
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let (n, m): (usize, usize) = line.next();
    let mut pokemon_list_sorted_by_id: Vec<Pokemon> = vec![];
    let mut pokemon_list_sorted_by_name = vec![];
    for i in 1..=n {
        let pokemon_name = line.next_str();
        let new_pokemon = Pokemon {
            id: i,
            name: pokemon_name.to_string(),
        };
        pokemon_list_sorted_by_id.push(new_pokemon.clone());
        pokemon_list_sorted_by_name.push(new_pokemon.clone());
    }

    pokemon_list_sorted_by_name.sort_by(|a, b| a.name.cmp(&b.name));

    for _i in 0..m {
        let input = line.next_str();

        match input.parse::<usize>() {
            Ok(num) => writeln!(buf_write, "{}", pokemon_list_sorted_by_id[num - 1].name).ok(),
            Err(_) => writeln!(
                buf_write,
                "{}",
                binary_search(&pokemon_list_sorted_by_name, input.to_string()).unwrap()
            )
            .ok(),
        };
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
