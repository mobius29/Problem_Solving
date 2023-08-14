// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
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
    let line = &mut read();

    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());

    let t: usize = line.next();

    for _test_case in 0..t {
        let (n, m): (usize, usize) = line.next();
        let priorities: Vec<usize> = line.next_iter().take(n).collect();

        let mut queue: Queue<(usize, usize)> = Queue::new();
        for i in 0..n {
            queue.enqueue((i, priorities[i]));
        }

        let mut priority_counts: [u32; 10] = [0; 10];
        for priority in priorities {
            priority_counts[priority] += 1;
        }

        let mut current_priority = 9;
        let mut count = 0usize;
        let mut flag = false;

        loop {
            if current_priority == 0 {
                break;
            }

            if priority_counts[current_priority] == 0 {
                current_priority -= 1;
                continue;
            }

            loop {
                if queue.empty() {
                    panic!("queue is empty!");
                }

                let (doc, prior) = queue.front().unwrap().to_owned();
                queue.dequeue().ok();
                if prior == current_priority {
                    count += 1;
                    if doc == m {
                        writeln!(buf_write, "{}", count).ok();
                        flag = true;
                    }
                    priority_counts[current_priority] -= 1;
                    break;
                }
                queue.enqueue((doc, prior));
            }

            if flag {
                break;
            }
        }
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
