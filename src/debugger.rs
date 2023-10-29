use crate::cpu::CPU;
use crate::memory::Memory;
use std::io::{stdin, stdout, Write};

pub struct Debugger {
    enabled: bool
}

enum Action {
    UNKNOWN,
    QUIT,
    STEP,
    CONTINUE,
}

impl Debugger {
    pub fn new(enabled: bool) -> Debugger{
        Debugger {
            enabled: enabled,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn get_next_user_action(&self) -> Action {
        let mut input_string = String::new();
        print!("DEBUGGER> ");
        let _ = stdout().flush();
        if let Ok(_) = stdin().read_line(&mut input_string) {
            match input_string.trim().to_uppercase().as_str() {
                "STEP" => {
                    Action::STEP
                },
                "QUIT" => {
                    Action::QUIT
                },
                "CONTINUE" => {
                    Action::CONTINUE
                },
                _ => {
                    Action::UNKNOWN
                }
            }
        }else{
            Action::UNKNOWN
        }
    }

    pub fn execute_next_user_action(&mut self, cpu: &mut CPU, mem: &mut Memory) {
        match self.get_next_user_action() {
            Action::UNKNOWN => {
                println!("Action: Action::UNKNOWN")
            },
            Action::QUIT => {
                println!("Action: Action::QUIT");
                cpu.set_halt();
            },
            Action::STEP => {
                println!("Action: Action::STEP");
                cpu.step(mem);
            },
            Action::CONTINUE => {
                println!("Action: Action::CONTINUE");
                //TODO: Note, we dont support breakpoints (yet) - so this just disables the debugger
                self.enabled = false;
            }
        }
    }
}
