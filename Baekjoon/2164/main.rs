use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: Next<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Box<Node<T>> {
        Box::new(Node { value, next: None })
    }
}

type Next<T> = Option<NonNull<Node<T>>>;

struct Queue<T> {
    front: Next<T>,
    back: Next<T>,
    size: u32,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            front: None,
            back: None,
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    fn get_node_ptr(node: Box<Node<T>>) -> Next<T> {
        NonNull::new(Box::into_raw(node))
    }

    pub fn enqueue(&mut self, value: T) {
        let new_node = Node::new(value);
        let new_node_ptr = Queue::get_node_ptr(new_node);

        match self.back {
            Some(back_ptr) => unsafe {
                (*back_ptr.as_ptr()).next = new_node_ptr;
                self.back = new_node_ptr;
            },
            None => self.front = new_node_ptr,
        };

        self.back = new_node_ptr;
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Result<(), &str> {
        match self.front {
            Some(front_ptr) => {
                unsafe {
                    self.front = (*front_ptr.as_ptr()).next;
                }

                self.size -= 1;
                if self.size == 0 {
                    self.back = None;
                }
                Ok(())
            }
            None => Err("queue is empty!"),
        }
    }

    pub fn front(&self) -> Result<&T, &str> {
        match self.front {
            Some(front_ptr) => {
                let return_value = unsafe { &(*front_ptr.as_ptr()).value };
                Ok(return_value)
            }
            None => Err("queue is empty!"),
        }
    }

    pub fn front_mut(&self) -> Result<&mut T, &str> {
        match self.front {
            Some(front_ptr) => {
                let return_value = unsafe { &mut (*front_ptr.as_ptr()).value };
                Ok(return_value)
            }
            None => Err("queue is empty!"),
        }
    }

    pub fn clear(&mut self) {
        while !self.empty() {
            self.dequeue().ok();
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let n: usize = line.next();
    let mut cards: Queue<usize> = Queue::new();
    for i in 1..=n {
        cards.enqueue(i);
    }

    while cards.size > 1 {
        cards.dequeue().ok();
        let front = *cards.front().unwrap();
        cards.dequeue().ok();

        cards.enqueue(front);
    }

    let answer = *cards.front().unwrap();
    writeln!(buf_write, "{}", answer).ok();

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
