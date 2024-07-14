use float_cmp::approx_eq;
use std::collections::VecDeque;
use std::ops::{Add, Sub};
use termion::{clear, color, cursor, style, terminal_size};

pub enum Mode {
    Dec,
    Int,
    Hex,
    Bin,
}

pub struct Stack<T> {
    pub stack_size: u16,
    pub stack: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new(stack_size: u16) -> Stack<T> {
        let mut stack: VecDeque<T> = VecDeque::new();
        Stack {
            stack_size: stack_size,
            stack: stack,
        }
    }

    /// Pushes new item to stack
    pub fn push(&mut self, item: T) {
        self.stack.push_front(item);
    }
}

impl<T: Add<Output = T>> Stack<T> {
    fn add(&mut self) {
        let x = self.stack.remove(0).unwrap();
        let y = self.stack.remove(0).unwrap();
        self.stack.push_front(x + y);
    }
}

impl<T: Sub<Output = T>> Stack<T> {
    fn sub(&mut self) {
        let x = self.stack.remove(0).unwrap();
        let y = self.stack.remove(0).unwrap();
        self.stack.push_front(y - x);
    }
}

pub fn render<T>(stack: Stack<T>) {
    let (columns, rows) = terminal_size().unwrap();
    for i in 1..stack.stack_size + 1 {
        print!("{}{:03}:", cursor::Goto(1, stack.stack_size + 1 - i), i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_int() {
        let mut stack: Stack<i64> = Stack::new(4);
        stack.push(5);
        stack.push(6);
        stack.add();
        let chk: i64 = 11;
        assert_eq!(stack.stack.get(0).unwrap().clone(), chk);
        assert_eq!(stack.stack.len(), 1);
    }

    #[test]
    fn test_add_float() {
        let mut stack: Stack<f64> = Stack::new(4);
        stack.push(1.0);
        stack.push(3.14);
        stack.add();
        assert!(approx_eq!(f64, stack.stack.get(0).unwrap().clone(), 4.14));
    }

    #[test]
    fn test_sub_int() {
        let mut stack: Stack<i64> = Stack::new(4);
        stack.push(10);
        stack.push(5);
        stack.sub();
        let chk: i64 = 5;
        assert_eq!(stack.stack.get(0).unwrap().clone(), chk);
    }
}
