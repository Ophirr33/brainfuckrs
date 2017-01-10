use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let args = std::env::args();
    if args.len() > 1 {
        // execute files
        use std::fs::File;
        args.skip(1)
            .map(|x| (File::open(x)
                      .unwrap()
                      .bytes()
                      .map(|x| x.unwrap()).collect::<Vec<u8>>(), [0; 30_000]))
            .fold((), |_, (bytes, mut mem)| { Interpreter::new(&bytes, &mut mem).interpret(); println!() });
    } else {
        let mut memory = [0; 30_000];
        let bytes = io::stdin().bytes().map(|x| x.unwrap()).collect::<Vec<u8>>();
        Interpreter::new(&bytes[..], &mut memory).interpret();
    }
}

struct Interpreter<'a> {
    memory: &'a mut [u8],
    code: &'a [u8],
    head: usize,
    brackets: HashMap<usize, usize>
}

const LEFT_BRACKET: u8 = '[' as u8;
const RIGHT_BRACKET: u8 = ']' as u8;

impl<'a> Interpreter<'a> {
    fn new(code: &'a [u8], memory: &'a mut [u8]) -> Self {
        Interpreter{ memory: memory, code: code, head: 0, brackets: Self::preprocess_code(code) }
    }

    fn preprocess_code(code: &'a [u8]) -> HashMap<usize, usize> {
        let count = code.iter().filter(|&x| *x == LEFT_BRACKET || *x == RIGHT_BRACKET).count();
        let mut hash_map = HashMap::with_capacity(count);
        let mut stack = Vec::with_capacity(count);
        for (i, byte) in code.iter().enumerate() {
            if *byte == LEFT_BRACKET {
                stack.push(i);
            } else if *byte == RIGHT_BRACKET && stack.is_empty() {
                panic!(format!("Unexpected right bracket at {}", i));
            } else if *byte == RIGHT_BRACKET {
                let left = stack.pop().unwrap();
                hash_map.insert(left, i);
                hash_map.insert(i, left);
            }
        }
        if !stack.is_empty() {
            panic!(format!("Unclosed left bracket at {}", stack.pop().unwrap()));
        }
        hash_map
    }

    fn interpret(&mut self) {
        let mut i = 0;
        while i < self.code.len() {
            match self.code[i] as char {
                '+' => self.memory[self.head] += 1,
                '-' => self.memory[self.head] -= 1,
                '<' => self.head -= 1,
                '>' => self.head += 1,
                ',' => self.memory[self.head] = io::stdin().bytes().next().unwrap().unwrap(),
                '.' => print!("{}", self.memory[self.head] as char),
                '[' if self.memory[self.head] == 0u8 => i = *self.brackets.get(&i).unwrap(),
                ']' if self.memory[self.head] != 0u8 => i = *self.brackets.get(&i).unwrap(),
                _ => ()
            }
            i += 1;
        }
    }
                    
}
