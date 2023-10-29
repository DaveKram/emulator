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
    PRINT_REGS
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
                "PRINT REGS" => {
                    Action::PRINT_REGS
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
            },
            Action::PRINT_REGS => {
                println!(
                    "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10} | {8: <10} | {9: <10} | {10: <10} | {11: <10} | {12: <10}",
                    "PC", "SP", "AC", "IX", "IY", "PS_CF", "PS_ZF", "PS_ID", "PS_DM", "PS_BC", "PS_OF", "PS_NF", "PS_UN"
                );
                println!(
                    "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10} | {8: <10} | {9: <10} | {10: <10} | {11: <10} | {12: <10}",
                    cpu.reg_pc, cpu.reg_sp, cpu.reg_accum, cpu.reg_index_x, cpu.reg_index_y, cpu.reg_ps_cf, cpu.reg_ps_zf, cpu.reg_ps_id, cpu.reg_ps_dm, cpu.reg_ps_bc, cpu.reg_ps_of, cpu.reg_ps_nf, cpu.reg_ps_un
                );
            }
        }
    }
}
