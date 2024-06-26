use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
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

fn print_stack(stackstruct: &Stack) {
    let mut stack: &mut VecDeque<f64> = &mut stackstruct.stack;
    let (columns, rows) = terminal_size().unwrap();
    print!(
        "{}{}{:?}",
        cursor::Goto(1, rows),
        clear::CurrentLine,
        stack
    );
    print!(
        "{}{}x{}",
        cursor::Goto(1, rows-1),
        columns,
        rows);
    for i in 1..stackstruct.stack_size + 1 {
        print!(
            "{}{:03}:",
            cursor::Goto(1, stackstruct.stack_size + 1 - i),
            i
        );
    }
    let mut line = 0;
    for value in stack.iter() {
        print!("{}{:>14.4}", cursor::Goto(7, stackstruct.stack_size - line), value);
        line += 1;
    }
    let _ = write!(
        stdout(),
        "{}{}> ",
        cursor::Goto(6, stackstruct.stack_size + 1),
        clear::CurrentLine,
    );
    stdout().flush();
}

fn operation(stack: &mut VecDeque<f64>, operation: char) {
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
    stack.resize(stack.len(), 0.0);
}

fn push_to_stack(stack: &mut VecDeque<f64>, line: &String) {
    let parse_result = line.parse::<f64>();
    let number = match parse_result {
        Ok(num) => {
            stack.pop_back();
            stack.push_front(num);
        }
        Err(error) => (),
    };
}

fn get_input(mut stack_str: Stack) -> String {
    let stack: &mut VecDeque<f64> = &mut stack_str.stack;
    let (columns, rows) = terminal_size().unwrap();
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
                    stack.resize(stack.len(), 0.0);
                    print_stack(&stack_str);
                }
                '+' | '-' | '*' | '/' | '%' => {
                    if line != "" {
                        push_to_stack(&mut stack, &line)
                    };
                    operation(&mut stack, c);
                    line.clear();
                    print_stack(&stack_str);
                }
                '<' => {
                    stack.swap(0, 1);
                    print_stack(&stack_str);
                }
                '\n' => {
                    push_to_stack(&mut stack, &line);
                    print_stack(&stack_str);
                    line.clear();
                }
                'i' => {
                    let _ = write!(
                        stdout,
                        "{}{}{}",
                        style::Invert,
                        cursor::Goto(1, rows - 2),
                        "INT"
                    );
                    let _ = write!(stdout, "{}", style::NoInvert);
                    print_stack(&stack_str);
                    line.clear();
                    stdout.flush().unwrap();
                },
                'd' => {
                    let _ = write!(
                        stdout,
                        "{}{}{}",
                        style::Invert,
                        cursor::Goto(1, rows - 2),
                        "DEC"
                    );
                    let _ = write!(stdout, "{}", style::NoInvert);
                    print_stack(&mut stack_str);
                    line.clear();
                    stdout.flush().unwrap();
                },
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
    let args: Vec<String> = std::env::args().collect();
    let stack_size: u16;
    if args.len() < 2 {
        stack_size = 4;
    }
    else {
        stack_size = args[1].parse::<u16>().unwrap();
    }
    print!("{}", clear::All);
    print!("{}", color::Fg(color::Black));
    let mut stack: Stack = Stack {
        stack_size: stack_size,
        mode: Mode::Dec,
        stack: VecDeque::from(vec![0.0; stack_size.into()])
    };
    print_stack(&stack);
    let line = get_input(stack);
}
