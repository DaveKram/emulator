use crate::cpu::CPU;
use crate::memory::Memory;
use std::io::{stdin, stdout, Write};

pub struct Debugger {
    enabled: bool,
    in_continue: bool
}

#[derive(PartialEq)]
enum Action {
    UNKNOWN,
    HELP,
    QUIT,
    STEP,
    CONTINUE,
    PRINT_REGS
}

impl Debugger {
    pub fn new(enabled: bool) -> Debugger{
        Debugger {
            enabled: enabled,
            in_continue: false
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn print_help(&self) {
        println!("\tHELP");
        println!("\t\tPrints out all available debugger commands and their usage");
        println!("\tSTEP");
        println!("\t\tIncrements, and then executes, the CPU program counter by one instruction");
        println!("\tQUIT");
        println!("\t\tSets the halt flag of the CPU, exiting on the next CPU cycle");
        println!("\tCONTINUE");
        println!("\t\tDisables the debugger and continues executing the CPU");
        println!("\tPRINT REGS");
        println!("\t\tPrints the CPU registers and their values");
    }

    fn get_next_user_action(&self) -> Action {
        let mut input_string = String::new();
        print!("DEBUGGER> ");
        let _ = stdout().flush();
        if let Ok(_) = stdin().read_line(&mut input_string) {
            match input_string.trim().to_uppercase().as_str() {
                "HELP" => {
                    Action::HELP
                },
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

    fn parse_and_execute_next_user_action(&mut self, cpu: &mut CPU, mem: &mut Memory) {
            match self.get_next_user_action(){
                Action::UNKNOWN => {
                    println!("DEBUGGER> Action: Action::UNKNOWN")
                },
                Action::HELP => {
                    self.print_help();
                },
                Action::QUIT => {
                    println!("DEBUGGER> Action: Action::QUIT");
                    cpu.set_halt();
                },
                Action::STEP => {
                    println!("DEBUGGER> Action: Action::STEP");
                    cpu.step(mem);
                },
                Action::CONTINUE => {
                    println!("DEBUGGER> Action: Action::CONTINUE");
                    self.in_continue = true;
                },
                Action::PRINT_REGS => {
                    println!(
                        "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10} | {8: <10} | {9: <10} | {10: <10} | {11: <10} | {12: <10}",
                        "PC", "SP", "AC", "IX", "IY", "PS_CF", "PS_ZF", "PS_ID", "PS_DM", "PS_BC", "PS_OF", "PS_NF", "PS_UN"
                    );
                    println!(
                        "{0: <10X} | {1: <10X} | {2: <10X} | {3: <10X} | {4: <10X} | {5: <10X} | {6: <10X} | {7: <10X} | {8: <10X} | {9: <10X} | {10: <10X} | {11: <10X} | {12: <10X}",
                        cpu.reg_pc, cpu.reg_sp, cpu.reg_accum, cpu.reg_index_x, cpu.reg_index_y, cpu.reg_ps_cf, cpu.reg_ps_zf, cpu.reg_ps_id, cpu.reg_ps_dm, cpu.reg_ps_bc, cpu.reg_ps_of, cpu.reg_ps_nf, cpu.reg_ps_un
                    );
                }
            }
    }

    pub fn execute_next_user_action(&mut self, cpu: &mut CPU, mem: &mut Memory) {
        if self.in_continue {
            //If we asked the debugger to continue, step the compiler like normally, until we hit a BRK
            if cpu.reg_ps_bc == 1 {
                println!("DEBUGGER> Hit Breakpoint!");
                self.in_continue = false;
                self.parse_and_execute_next_user_action(cpu, mem);
            }else{
                cpu.step(mem);
            }
        }else{
            //This is normal debugger operation, so always ask for input each time
            self.parse_and_execute_next_user_action(cpu, mem);
        }
    }
}
