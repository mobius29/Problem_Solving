use std::io::Write;
use std::ptr::NonNull;
use std::str::SplitAsciiWhitespace;
use std::{fmt, io};

struct Node<T> {
    value: T,
    prev: Next<T>,
    next: Next<T>,
}

impl<T> Node<T> {
    pub fn new(value: T, prev: Next<T>, next: Next<T>) -> Self {
        Self { value, prev, next }
    }
}

type Next<T> = Option<NonNull<Node<T>>>;

struct Deque<T> {
    front: Next<T>,
    rear: Next<T>,
    size: usize,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            rear: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, None, self.front));
        let new_node_ptr = NonNull::new(Box::into_raw(new_node));

        match self.front {
            None => self.rear = new_node_ptr,
            Some(front_ptr) => unsafe {
                (*front_ptr.as_ptr()).prev = new_node_ptr;
            },
        };

        self.front = new_node_ptr;
        self.size += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.rear, None));
        let new_node_ptr = NonNull::new(Box::into_raw(new_node));

        match self.rear {
            None => self.front = new_node_ptr,
            Some(rear_ptr) => unsafe {
                (*rear_ptr.as_ptr()).next = new_node_ptr;
            },
        };

        self.rear = new_node_ptr;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Result<(), String> {
        match self.front {
            None => Err("This Deque is empty!".to_string()),
            Some(front_ptr) => unsafe {
                let next_ptr = (*front_ptr.as_ptr()).next;
                match next_ptr {
                    None => {
                        self.rear = None;
                    }
                    Some(next) => {
                        (*next.as_ptr()).prev = None;
                    }
                }
                self.front = next_ptr;
                self.size -= 1;

                Ok(())
            },
        }
    }

    pub fn pop_back(&mut self) -> Result<(), String> {
        match self.rear {
            None => Err("This Deque is empty!".to_string()),
            Some(rear_ptr) => unsafe {
                let prev_ptr = (*rear_ptr.as_ptr()).prev;
                match prev_ptr {
                    None => {
                        self.front = None;
                    },
                    Some(prev) => {
                        (*prev.as_ptr()).next = None;
                    }
                }
                self.rear = prev_ptr;
                self.size -= 1;

                Ok(())
            },
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn front(&self) -> Result<&T, String> {
        match self.front.as_ref() {
            None => Err("This Deque is empty!".to_string()),
            Some(front_ptr) => unsafe { Ok(&(*front_ptr.as_ptr()).value) },
        }
    }

    pub fn back(&self) -> Result<&T, String> {
        match self.rear.as_ref() {
            None => Err("This Deque is empty!".to_string()),
            Some(rear_ptr) => unsafe { Ok(&(*rear_ptr.as_ptr()).value) },
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());
    let line = &mut read();

    let n: usize = line.next();
    let mut deque: Deque<usize> = Deque::new();

    for _i in 0..n {
        let str = line.next_str();
        if str == "push_front" {
            let num: usize = line.next();
            deque.push_front(num);
        }

        if str == "push_back" {
            let num: usize = line.next();
            deque.push_back(num);
        }

        if str == "pop_front" {
            match deque.front() {
                Err(_) => {
                    writeln!(buf_write, "-1").ok();
                }
                Ok(value) => {
                    writeln!(buf_write, "{}", value).ok();
                    deque.pop_front().ok();
                }
            };
        }

        if str == "pop_back" {
            match deque.back() {
                Err(_) => {
                    writeln!(buf_write, "-1").ok();
                }
                Ok(value) => {
                    writeln!(buf_write, "{}", value).ok();
                    deque.pop_back().ok();
                }
            };
        }

        if str == "size" {
            writeln!(buf_write, "{}", deque.size()).ok();
        }

        if str == "empty" {
            writeln!(buf_write, "{}", if deque.empty() { 1 } else { 0 }).ok();
        }

        if str == "front" {
            match deque.front() {
                Err(_) => {
                    writeln!(buf_write, "-1").ok();
                }
                Ok(value) => {
                    writeln!(buf_write, "{}", value).ok();
                }
            };
        }

        if str == "back" {
            match deque.back() {
                Err(_) => {
                    writeln!(buf_write, "-1").ok();
                }
                Ok(value) => {
                    writeln!(buf_write, "{}", value).ok();
                }
            };
        }
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
