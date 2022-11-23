extern crate termios;
use std::path::Path;
use std::{env};
use std::fs::File;
use std::io;

use std::io::Read;
use terminal::{Action, Retrieved, Value, Clear};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn print_screen(line : usize, string_buffer : &String ){
    let terminal = terminal::stdout();
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
    let height = if let Retrieved::TerminalSize(_, height) = terminal.get(Value::TerminalSize).unwrap() {
        height 
       }
       else {
           panic!();
       };

       for (index, line_string) in string_buffer.lines().enumerate() {
        if index < line {
            continue;
        } 
        println!("{}", line_string);

        if index > line + height as usize{
            break;
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("need more argument");
        return;
    }

    let filename = args[1].as_str();
    
    
    let string_buffer = match cat(Path::new(filename)) {
        Err(why) => panic!("{}", why.to_string()),
        Ok(s) => s,
    };
    
    print_screen(0, &string_buffer);
    
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios 
                                            // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    
    let mut current_line = 0;
    loop {
        reader.read_exact(&mut buffer).unwrap();    
        
        if buffer[0] == 65 {
            if current_line > 0 {
                current_line -= 1;
            } 
            print_screen(current_line, &string_buffer);
        }
        else if buffer[0] == 66 {
            
            if string_buffer.lines().count() > current_line {
                current_line += 1;
            } 
            print_screen(current_line, &string_buffer);
        }
    }
  
}
