use std::collections::VecDeque;
use termion::{clear, color, cursor, style, terminal_size};

enum Mode {
    Dec,
    Int,
    Hex,
    Bin,
}

struct Stack {
    stack_size: u16,
    mode: Mode,
    stack: VecDeque<f64>,
}

impl Stack {

    fn new() {
        Stack {
            stack_size: 4,
            mode: Mode::Dec,
            stack: VecDeque::from(vec![0.0, stack_size.into()]);
        }
    }
    
    fn print() {
        print_stack();
    }

    fn print_stack() {
        let (columns, rows) = terminal_size().unwrap();
        for i in 1..stackstruct.stack_size + 1 {
            print!(
                "{}{:03}:",
                cursor::Goto(1, stackstruct.stack_size + 1 - i),
                i
            );
        }
    }
}

    
