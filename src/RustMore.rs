extern crate termios;
use std::path::Path;
use std::{env};
use std::fs::File;
use std::io;

use std::io::Read;
use screen_manager::ScreenManager;
use screen_manager::command::ActionCommands;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

mod screen_manager;

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn ReadBufferToActionCommand() -> ActionCommands {

}

impl rust_more {
    pub fn Run(){

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
        
        let mut screen_manager = ScreenManager::build_screen_manager(string_buffer);
    
        screen_manager.print_screen();
        
        let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
                       // on /dev/stdin or /dev/tty
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();  // make a mutable copy of termios 
                                                // that we will modify
        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        
        let mut reader = io::stdin();
        let mut buffer = [0;1];  // read exactly one byte
        
        loop {
            reader.read_exact(&mut buffer).unwrap();    
            
            if buffer[0] == 65 {
                screen_manager.action(ActionCommands::Up(1));
                screen_manager.print_screen();
            }
            else if buffer[0] == 66 {
                screen_manager.action(ActionCommands::Down(1));
                screen_manager.print_screen();
            }
        }
    }
}