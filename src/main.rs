use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, input};

const BOTTOM_LINE: u16 = 24;
const STACK_SIZE_SCR: u16 = 10;
const STACK_SIZE: usize = 10;

fn print_stack(stack: &VecDeque<f64>) {
    print!("{}{}{:?}", cursor::Goto(1, BOTTOM_LINE), clear::CurrentLine, stack);
    for i in 1..STACK_SIZE_SCR + 1 {
        print!(
            "{}{:03}{}",
            cursor::Goto(1, STACK_SIZE_SCR + 1 - i),
            i,
            "             "
        );
    }
    let mut line = 0;
    for value in stack.iter() {
        print!("{}{:>7.4}", cursor::Goto(7, STACK_SIZE_SCR - line), value);
        line += 1;
    }
    write!(
        stdout(),
        "{}{}> ",
        cursor::Goto(6, STACK_SIZE_SCR + 1),
        clear::CurrentLine,
    );
    stdout().flush();
}

fn make_operation(stack: &mut VecDeque<f64>, line: &String, operation: char) {
    let arg1 = stack.pop_front().unwrap();
    let arg2 = line.parse::<f64>().unwrap();
    match operation {
        '*' => {
            stack.push_front(arg1 * arg2);
            stack.truncate(STACK_SIZE);
        }
        '-' => {
            stack.push_front(arg1 - arg2);
            stack.truncate(STACK_SIZE);
        }
        '/' => {
            stack.push_front(arg1 / arg2);
            stack.truncate(STACK_SIZE);
        }
        '+' => {
            stack.push_front(arg1 + arg2);
            stack.truncate(STACK_SIZE);
        }
        _ => todo!(),
    }
    print_stack(&stack);
}

fn operation(stack: &mut VecDeque<f64>, operation: char) {
    let arg1 = stack.pop_front().unwrap();
    let arg2 = stack.pop_front().unwrap();
    match operation {
        '+' => stack.push_front(arg2*arg1),
        _ => todo!(),
    }
}

fn push_to_stack(stack: &mut VecDeque<f64>, line: &String) {
    let parse_result = line.parse::<f64>();
    let number = match parse_result {
        Ok(num) => { stack.pop_back(); stack.push_front(num); },
        Err(error) => (),
    };
}

fn get_input(mut stack: VecDeque<f64>) -> String {
    let mut stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut line = String::new();
    for key in stdin.keys() {
        let c = key.unwrap();
        match c {
            Key::Char('q') => break,
            Key::Char('p') => {
                stack.pop_front();
                print_stack(&stack);
            }
            Key::Char('+') => {
                if line != "" {
                    push_to_stack(&mut stack, &line)
                };
                operation(&mut stack, c);
                line.clear();
                print_stack(&stack);
            }
            Key::Char('-') => {
                make_operation(&mut stack, &line, '-');
                line.clear()
            }
            Key::Char('*') => {
                make_operation(&mut stack, &line, '*');
                line.clear()
            }
            Key::Char('/') => {
                make_operation(&mut stack, &line, '/');
                line.clear()
            }
            Key::Char('\n') => {
                push_to_stack(&mut stack, &line);
                print_stack(&stack);
                line.clear();
            }
            Key::Char('i') => {
                write!(stdout, "{} {}", cursor::Goto(1, BOTTOM_LINE - 1), "INT");
                line.clear();
                stdout.flush().unwrap();
            }
            Key::Char(key) => {
                write!(stdout, "{}", key);
                stdout.flush().unwrap();
                line.push(key);
            }
            _ => todo!(),
        }
    }
    return line;
}

fn main() {
    print!("{}", clear::All);
    print!("{}", color::Fg(color::Red));
    let mut stack: VecDeque<f64> = VecDeque::from(vec![0.0;10]);
    print_stack(&stack);
    let line = get_input(stack);
}
