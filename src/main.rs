use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style, terminal_size};

const BOTTOM_LINE: u16 = 24;
const STACK_SIZE: u16 = 12;

enum Mode {
    Dec,
    Int,
}

struct Stack<T> {
    mode: Mode,
    stack: VecDeque<T>,
}

impl Stack<f64> {
    fn new() -> Stack<f64> {
        Stack {
            mode: Mode::Dec,
            stack: VecDeque::from(vec![0.0, STACK_SIZE.into()]),
        }
    }
}

impl Stack<i64> {
    fn new() -> Stack<i64> {
        Stack {
            mode: Mode::Dec,
            stack: VecDeque::from(vec![0, STACK_SIZE.into()]),
        }
    }
}

fn print_stack<T>(stack: &VecDeque<T>) {
    let (columns, rows) = terminal_size().unwrap();
    print!(
        "{}{}{:?}",
        cursor::Goto(1, columns),
        clear::CurrentLine,
        stack
    );
    for i in 1..STACK_SIZE + 1 {
        print!("{}{:03}:", cursor::Goto(1, STACK_SIZE + 1 - i), i);
    }
    let mut line = 0;
    for value in stack.iter() {
        print!("{}{:>12.4}", cursor::Goto(7, STACK_SIZE - line), value);
        line += 1;
    }
    let _ = write!(
        stdout(),
        "{}{}> ",
        cursor::Goto(6, STACK_SIZE + 1),
        clear::CurrentLine,
    );
    stdout().flush();
}

fn operation<T>(stack_struct: &mut VecDeque<T>, operation: char) {
    stack = stack_struct.stack;
    let arg1 = stack.pop_front().unwrap();
    let arg2 = stack.pop_front().unwrap();
    match operation {
        '+' => stack.push_front(arg2 + arg1),
        '-' => stack.push_front(arg2 - arg1),
        '*' => stack.push_front(arg2 * arg1),
        '/' => stack.push_front(arg2 / arg1),
        '%' => stack.push_front(arg2 % arg1),
        _ => todo!(),
    }
    stack.resize(STACK_SIZE.into(), 0.0);
}

fn push_to_stack<T>(stack: &mut VecDeque<T>, line: &String) {
    let parse_result = line.parse::<f64>();
    let number = match parse_result {
        Ok(num) => {
            stack.pop_back();
            stack.push_front(num);
        }
        Err(error) => (),
    };
}

fn get_input<T>(mut stack: VecDeque<T>) -> String {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut line = String::new();
    for key in stdin.keys() {
        if let Key::Char(c) = key.unwrap() {
            match c {
                'q' => {
                    write!(stdout, "{}{}", cursor::Goto(1, 1), clear::All);
                    stdout.flush().unwrap();
                    break;
                }
                'p' => {
                    stack.pop_front();
                    stack.resize(STACK_SIZE.into(), 0.0);
                    print_stack(&stack);
                }
                '+' | '-' | '*' | '/' | '%' => {
                    if line != "" {
                        push_to_stack(&mut stack, &line)
                    };
                    operation(&mut stack, c);
                    line.clear();
                    print_stack(&stack);
                }
                '<' => {
                    stack.swap(0, 1);
                    print_stack(&stack);
                }
                '\n' => {
                    push_to_stack(&mut stack, &line);
                    print_stack(&stack);
                    line.clear();
                }
                'i' => {
                    let _ = write!(
                        stdout,
                        "{}{}{}",
                        style::Invert,
                        cursor::Goto(1, BOTTOM_LINE - 1),
                        "INT"
                    );
                    let _ = write!(stdout, "{}", style::NoInvert);
                    print_stack(&stack);
                    line.clear();
                    stdout.flush().unwrap();
                }
                'd' => {
                    let _ = write!(
                        stdout,
                        "{}{}{}",
                        style::Invert,
                        cursor::Goto(1, BOTTOM_LINE - 1),
                        "DEC"
                    );
                    let _ = write!(stdout, "{}", style::NoInvert);
                    print_stack(&stack);
                    line.clear();
                    stdout.flush().unwrap();
                }
                _ => {
                    let _ = write!(stdout, "{}", c);
                    stdout.flush().unwrap();
                    line.push(c);
                }
            }
        }
    }
    return line;
}

fn main() {
    print!("{}", clear::All);
    print!("{}", color::Fg(color::Black));
    let mut stack: Stack<i64> = Stack::<i64>::new();
    print_stack(&stack.stack);
    let line = get_input(stack.stack);
}
