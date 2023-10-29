use crate::memory::Memory;

#[derive(Debug)]
enum InstructionTypes {
    LDA_IMMEDIATE
}

enum Error {
    FETCH_ERROR,
    FETCH_ERROR_UNKNOWN_INST
}

pub struct CPU {
    reg_pc: u16,            //Program counter
    reg_sp: u8,             //Stack pointer
    reg_accum: u8,          //Accumulator
    reg_index_x: u8,        //Index Register X
    reg_index_y: u8,        //Index Register Y
    reg_ps: u8,             //Procssor Status
    do_halt: bool           //To halt or not
}

#[derive(Debug)]
struct Instruction {
    inst: InstructionTypes,
    data: u16,
    num_cycles: u16
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg_pc: 0,
            reg_sp: 0,
            reg_accum: 0,
            reg_index_x: 0,
            reg_index_y: 0,
            reg_ps: 0,
            do_halt: false
        }
    }

    pub fn check_halt(&self) -> bool {
        self.do_halt
    }

    pub fn set_halt(&mut self) {
        self.do_halt = true;
    }

    fn fetch(&self, mem: &Memory) -> Result<Instruction, Error> {
        if let Ok(inst) = mem.read_inst(self.reg_pc) {
            match inst & 0xFF00 {
                0xA900 => {
                    Ok(Instruction { inst: InstructionTypes::LDA_IMMEDIATE, data: inst, num_cycles: 2 })
                }
                _ => {
                    Err(Error::FETCH_ERROR_UNKNOWN_INST)
                }
            }
        }else{
            Err(Error::FETCH_ERROR)
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst.inst {
            InstructionTypes::LDA_IMMEDIATE => {
                println!("Instruction: LDA Immediate");
                self.reg_accum = (inst.data & 0x00FF) as u8;
            }
            _ => {
                println!("Unknown instruction! Instruction: {:?}", inst);
            }
        }
    }

    pub fn step(&mut self, mem_ref: &Memory) {
        //Fetch the next instruction and then number of cycles it takes
        if let Ok(inst) = self.fetch(mem_ref) {
            //Excute the instuction
            self.execute(&inst);

            //Increment the PC
            self.reg_pc += inst.num_cycles;
        }
    }
}