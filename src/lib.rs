use float_cmp::approx_eq;
use std::vec::Vec;
use std::ops::{Add, Sub};
use std::io::stdout;
use termion::{clear, color, cursor, style, terminal_size};
use termion::raw::{IntoRawMode};

pub enum Mode {
    Dec,
    Int,
    Hex,
    Bin,
}

#[derive(Debug)]
pub struct Stack<T> {
    pub stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        let mut stack: Vec<T> = Vec::new();
        Stack {
            stack: stack,
        }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }
}

impl<T: Add<Output = T>> Stack<T> {
    fn add(&mut self) {
        let x = self.stack.pop().unwrap();
        let y = self.stack.pop().unwrap();
        self.stack.push(x + y);
    }
}

impl<T: Sub<Output = T>> Stack<T> {
    fn sub(&mut self) {
        let x = self.stack.pop().unwrap();
        let y = self.stack.pop().unwrap();
        self.stack.push(y - x);
    }
}

pub fn render<T>(stack: Stack<T>) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (columns, rows) = terminal_size().unwrap();
    let stack_size:u16 = stack.stack.len() as u16;
    for i in 1..stack_size+1 {
        print!("{}{:03}:", cursor::Goto(1, stack_size + 1 - i), i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_int() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(5);
        stack.push(6);
        stack.add();
        let chk: i64 = 11;
        assert_eq!(stack.stack.get(0).unwrap().clone(), chk);
        assert_eq!(stack.stack.len(), 1);
    }

    #[test]
    fn test_add_float() {
        let mut stack: Stack<f64> = Stack::new();
        stack.push(1.0);
        stack.push(3.14);
        stack.add();
        assert!(approx_eq!(f64, stack.stack.get(0).unwrap().clone(), 4.14));
    }

    #[test]
    fn test_sub_int() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(10);
        stack.push(5);
        stack.sub();
        let chk: i64 = 5;
        assert_eq!(stack.stack.get(0).unwrap().clone(), chk);
    }
}
