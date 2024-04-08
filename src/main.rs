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

fn get_input(mut stack: VecDeque<f64>) -> String {
    let mut stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut line = String::new();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') => break,
            Key::Char('p') => {
                stack.pop_front();
                print_stack(&stack);
            }
            Key::Char('+') => {
                make_operation(&mut stack, &line, '+');
                line.clear()
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
                stack.push_front(line.parse::<f64>().unwrap());
                stack.truncate(STACK_SIZE);
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
    let mut stack: VecDeque<f64> = VecDeque::with_capacity(10);
    print_stack(&stack);
    let line = get_input(stack);
    println!("{}", line);
}
