// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

struct Stack<T> {
    head: NextNode<T>,
    size: usize,
}

struct Node<T> {
    value: T,
    next_node: NextNode<T>,
}

type NextNode<T> = Option<Box<Node<T>>>;

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next_node: self.head.take(),
        });

        self.size += 1;
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Result<(), &str> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next_node;
                self.size -= 1;

                Ok(())
            }
            None => Err("Cannot pop this stack because stack is empty."),
        }
    }

    pub fn top(&mut self) -> Result<&T, &str> {
        match self.head.as_ref() {
            Some(node) => Ok(&node.value),
            None => Err("Cannot get value of this stack because stack is empty."),
        }
    }

    pub fn top_mut(&mut self) -> Result<&mut T, &str> {
        match self.head.as_mut() {
            Some(node) => Ok(&mut node.value),
            None => Err("Cannot get value of this stack because stack is empty."),
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn clear(&mut self) -> () {
        while !self.empty() {
            self.pop().unwrap();
        }
    }
}

fn main() {
    let line = &mut read();

    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());

    let mut stack: Stack<usize> = Stack::new();

    let n: usize = line.next();
    let result: Vec<usize> = line.next_iter().take(n).collect();

    let mut current_count = 1;
    let mut answer: Vec<char> = Vec::new();

    let mut is_valid = true;
    for i in 0..n {
        while current_count <= result[i] {
            stack.push(current_count);
            current_count += 1;

            answer.push('+');
        }

        let top = stack.top().unwrap();
        if *top == result[i] {
            stack.pop().unwrap();
            answer.push('-');
        } else {
            is_valid = false;
            break;
        }
    }

    if is_valid {
        for c in answer {
            writeln!(buf_write, "{}", c).ok();
        }
    } else {
        writeln!(buf_write, "NO").ok();
    }

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

fn read() -> Tokenizer<SplitAsciiWhitespace<'static>> {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let str: &'static str = Box::leak(buf.into_boxed_str());

    Tokenizer::new(str, |s| s.split_ascii_whitespace())
}
