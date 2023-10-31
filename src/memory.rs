use core::num;
use std::fs::File;
use std::io::Read;

const MAX_MEMORY_SIZE_BYTES: usize = 65536; //Max size is the fact the 6502 has an 8 bit accumulator
const STACK_START: u16 = 0x0100;
const STACK_END: u16 = 0x01FF;

pub struct Memory {
    mem: [u8; MAX_MEMORY_SIZE_BYTES],
}

#[derive(Debug)]
pub enum Error {
    READ_OUT_OF_BOUNDS,
    WRITE_OUT_OF_BOUNDS,
    PROGRAM_SIZE_TOO_LARGE,
    FILE_NOT_FOUND,
    PUSH_ON_STACK_OUT_OF_SPACE,
    POP_OFF_STACK_EMPTY,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; MAX_MEMORY_SIZE_BYTES],
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, Error> {
        let index = addr as usize;
        if index > MAX_MEMORY_SIZE_BYTES
            || (index >= (STACK_START as usize) && index <= (STACK_END as usize))
        {
            Err(Error::READ_OUT_OF_BOUNDS)
        } else {
            Ok(self.mem[index])
        }
    }

    pub fn read_n_bytes(&self, addr: u16, size: usize) -> Result<Vec<u8>, Error> {
        let mut output: Vec<u8> = Vec::new();
        for i in 0..size {
            if let Ok(byte) = self.read_byte(addr + (i as u16)) {
                output.push(byte);
            } else {
                return Err(Error::READ_OUT_OF_BOUNDS);
            }
        }
        Ok(output)
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) -> Result<u8, Error> {
        let index = addr as usize;
        if index > MAX_MEMORY_SIZE_BYTES
            || (index >= (STACK_START as usize) && index <= (STACK_END as usize))
        {
            Err(Error::WRITE_OUT_OF_BOUNDS)
        } else {
            self.mem[index] = data;
            Ok(self.mem[index])
        }
    }

    fn load_program_bytes(&mut self, start_addr: u16, bytes: &Vec<u8>) -> Result<(), Error> {
        let start_index: usize = start_addr as usize;
        if start_index > MAX_MEMORY_SIZE_BYTES
            || (start_index >= (STACK_START as usize) && start_index <= (STACK_END as usize))
            || (start_index + bytes.len() + 1 >= (STACK_START as usize)
                && start_index + bytes.len() + 1 <= (STACK_END as usize))
        {
            Err(Error::WRITE_OUT_OF_BOUNDS)
        } else {
            if (start_index + bytes.len() + 1 > (MAX_MEMORY_SIZE_BYTES as usize)) {
                Err(Error::PROGRAM_SIZE_TOO_LARGE)
            } else {
                //Put program into memory
                self.mem[start_index..(start_index + bytes.len())]
                    .copy_from_slice(bytes.as_slice());
                //Put a BRK instruction at the end of the program
                self.mem[(start_addr as usize) + bytes.len() + 1] = 0x0;
                Ok(())
            }
        }
    }

    pub fn load_program_from_file(&mut self, start_addr: u16, filename: &str) -> Result<(), Error> {
        if let Ok(mut f) = File::open(&filename) {
            if let Ok(metadata) = std::fs::metadata(&filename) {
                let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
                if let Ok(_) = f.read(&mut buffer) {
                    //Load the main program
                    self.load_program_bytes(start_addr, &buffer)
                } else {
                    Err(Error::PROGRAM_SIZE_TOO_LARGE)
                }
            } else {
                Err(Error::FILE_NOT_FOUND)
            }
        } else {
            Err(Error::FILE_NOT_FOUND)
        }
    }

    pub fn push_onto_stack(&mut self, stack_pointer: u8, bytes: &Vec<u8>) -> Result<(), Error> {
        let start_index = (STACK_END as usize) - ((STACK_END as usize) - (STACK_START as usize + stack_pointer as usize));
        if start_index - bytes.len() < STACK_START as usize {
            Err(Error::PUSH_ON_STACK_OUT_OF_SPACE)
        } else {
            let mut cur_index = start_index;
            for b in bytes {
                self.mem[cur_index] = b.clone();
                //TODO: Remove this debug print
                println!("{:#04x} at {:#04x}", self.mem[cur_index], cur_index);
                cur_index -= 1;
            }   
            Ok(())
        }
    }

    pub fn pop_off_stack(&mut self, stack_pointer: u8, num_bytes: u8) -> Result<Vec<u8>, Error> {
        //TODO:
        Err(Error::POP_OFF_STACK_EMPTY)
    }
}
