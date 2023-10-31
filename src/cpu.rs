use crate::memory::Memory;

#[derive(Debug)]
enum InstructionTypes {
    BRK,
    LDA_IMMEDIATE,
    STA_ABSOLUTE,
    ASL_ACCUMULATOR,
    ADC_IMMEDIATE,
    ROL_IMMEDIATE
}

enum Error {
    FETCH_ERROR,
    FETCH_ERROR_UNKNOWN_INST
}

pub struct CPU {
    pub reg_pc: u16,            //Program counter
    pub reg_sp: u8,             //Stack pointer
    pub reg_accum: u8,          //Accumulator
    pub reg_index_x: u8,        //Index Register X
    pub reg_index_y: u8,        //Index Register Y
    pub reg_ps_cf: u8,          //Processor Status carry flag
    pub reg_ps_zf: u8,          //Processor Status zero flag
    pub reg_ps_id: u8,          //Processor Status interrupt disable
    pub reg_ps_dm: u8,          //Processor Status decimal mode
    pub reg_ps_bc: u8,          //Processor Status break command
    pub reg_ps_of: u8,          //Processor Status overflow flag
    pub reg_ps_nf: u8,          //Processor Status negative flag
    pub reg_ps_un: u8,          //Processor Status unused flag
    do_halt: bool               //To halt or not
}

#[derive(Debug)]
struct Instruction {
    inst: InstructionTypes,
    data: Vec<u8>,
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
            reg_ps_cf: 0,
            reg_ps_zf: 1,
            reg_ps_id: 1,
            reg_ps_dm: 0,
            reg_ps_bc: 0,
            reg_ps_of: 0,
            reg_ps_nf: 0,
            reg_ps_un: 1,
            do_halt: false
        }
    }

    pub fn check_halt(&self) -> bool {
        self.do_halt
    }

    pub fn set_halt(&mut self) {
        self.do_halt = true;
    }

    fn update_status_regs(&mut self, res: u8) 
    {
        //Reset BRK flag if it was 1 - reg_ps_bc
        if self.reg_ps_bc == 1 {
            self.reg_ps_bc = 0;
        }

        //Set negative flag - reg_ps_nf
        //If high bit set, then negative flag is set
        if (res & 0x80) == 0x80 {
            self.reg_ps_nf = 1;
        }else{
            self.reg_ps_nf = 0;
        }

        //Set zero flag - reg_ps_zf
        //If res is 0, then flag is 1, otherwise it is 0
        if res == 0 {
            self.reg_ps_zf = 1;
        }else{
            self.reg_ps_zf = 0;
        }
    }

    fn fetch(&self, mem: &Memory) -> Result<Instruction, Error> {
        if let Ok(inst) = mem.read_byte(self.reg_pc) {
            match inst {
                0x0 => {
                    //1 bytes total
                    Ok(Instruction { inst: InstructionTypes::BRK, data: vec![inst], num_cycles: 7 })
                }
                0xA9 => {
                    // 2 bytes total
                    if let Ok(data_bytes) = mem.read_n_bytes(self.reg_pc + 1, 1) {
                        Ok(Instruction { inst: InstructionTypes::LDA_IMMEDIATE, data: data_bytes, num_cycles: 2 })
                    }else{
                        Err(Error::FETCH_ERROR)
                    }
                },
                0x8D => {
                    // 3 bytes total
                    if let Ok(data_bytes) = mem.read_n_bytes(self.reg_pc + 1, 2) {
                        Ok(Instruction { inst: InstructionTypes::STA_ABSOLUTE, data: data_bytes, num_cycles: 4 })
                    }else{
                        Err(Error::FETCH_ERROR)
                    }
                },
                0x0A => {
                    // 1 bytes total
                    Ok(Instruction { inst: InstructionTypes::ASL_ACCUMULATOR, data: vec![inst], num_cycles: 2 })
                },
                0x69 => {
                    // 2 bytes total
                    if let Ok(data_bytes) = mem.read_n_bytes(self.reg_pc + 1, 1) {
                        Ok(Instruction { inst: InstructionTypes::ADC_IMMEDIATE, data: data_bytes, num_cycles: 2 })
                    }else{
                        Err(Error::FETCH_ERROR)
                    }
                },
                0x2A => {
                    // 1 bytes total
                    Ok(Instruction { inst: InstructionTypes::ROL_IMMEDIATE, data: vec![inst], num_cycles: 2 })
                }
                _ => {
                    println!("CPU> Unknown instruction. Opcode: {:#04x}", inst);
                    Err(Error::FETCH_ERROR_UNKNOWN_INST)
                }
            }
        }else{
            Err(Error::FETCH_ERROR)
        }
    }

    fn execute(&mut self, inst: &Instruction, mem: &mut Memory) {
        match inst.inst {
            InstructionTypes::BRK => {
                println!("CPU> Instruction: BRK");
                //TODO: More than this
                self.reg_ps_bc = 1;
                self.reg_pc += 1;
            },
            InstructionTypes::LDA_IMMEDIATE => {
                println!("CPU> Instruction: LDA Immediate");
                self.reg_accum = inst.data[0] as u8;
                self.update_status_regs(self.reg_accum);
                self.reg_pc += 2;
            },
            InstructionTypes::STA_ABSOLUTE => {
                println!("CPU> Instruction: STA Absolute");
                if let Ok(_) = mem.write_byte((inst.data[0] as u16) | ((inst.data[1] as u16) << 8), self.reg_accum) {
                    self.update_status_regs(self.reg_accum);
                    self.reg_pc += 3;
                }else{
                    //TODO: Read fails?
                }
            },
            InstructionTypes::ASL_ACCUMULATOR => {
                println!("CPU> Instruction: ASL Accumulator");
                self.reg_accum <<= self.reg_accum; //TODO: Is this right?
                self.update_status_regs(self.reg_accum);
                self.reg_pc += 1;
            },
            InstructionTypes::ADC_IMMEDIATE => {
                println!("CPU> Instruction: ADC Immediate");
                self.reg_accum += inst.data[0] as u8;
                self.update_status_regs(self.reg_accum);
                self.reg_pc += 2;
            },
            InstructionTypes::ROL_IMMEDIATE => {
                println!("CPU> Instruction: ROL Immediate");
                self.reg_accum <<= 1; //TODO: Is this right?
                self.update_status_regs(self.reg_accum);
                self.reg_pc += 1;
            }
        }
    }

    pub fn step(&mut self, mem_ref: &mut Memory) {
        //Fetch the next instruction and then number of cycles it takes
        if let Ok(inst) = self.fetch(mem_ref) {
            //Excute the instuction
            self.execute(&inst, mem_ref);
        }else{
            println!("CPU> Failed to fetch next instruction!");
        }
    }
}