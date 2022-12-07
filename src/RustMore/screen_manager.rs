use terminal::{Action, Retrieved, Value, Clear};
use std::cmp;
use std::process;

pub mod command;

pub struct ScreenManager {
    current_line : usize,
    string_buffer : String,
    //current_mode : more 옵션 추가개발?
}

impl ScreenManager {
    pub fn build_screen_manager(string_buffer: String) -> ScreenManager {
        ScreenManager { 
            current_line: 0, 
            string_buffer: string_buffer 
        }
    }

    pub fn action(&mut self, command: command::ActionCommands) {
        match command {
            command::ActionCommands::Move(line) => {
                self.current_line = line;
            },
            command::ActionCommands::Up(lines) => {
                if self.current_line < lines {
                    self.current_line = 0;
                }
                else
                {
                    self.current_line -= lines;
                }
            }
            command::ActionCommands::Down(lines) => {
                let line_count  =self.string_buffer.lines().count();

                if line_count > self.current_line + lines {
                    self.current_line += lines;
                }
                else
                {
                    self.current_line = line_count - 1;
                }
            }
            command::ActionCommands::Quit => {
                process::exit(0);
            },
        }
    }

    pub fn print_screen(&self) {
        let terminal = terminal::stdout();
        terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
        let height = if let Retrieved::TerminalSize(_, height) = terminal.get(Value::TerminalSize).unwrap() {
            height 
           }
           else {
               panic!();
           };
    
        for (index, line_string) in self.string_buffer.lines().enumerate().skip(self.current_line).take(height as usize) {
            println!("{}", line_string);
        }
    }
}

