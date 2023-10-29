use std::fs::File;
use std::io::Read;

const MAX_MEMORY_SIZE_BYTES: usize = 65536; //Max size is the fact the 6502 has an 8 bit accumulator

pub struct Memory {
    mem: [u8; MAX_MEMORY_SIZE_BYTES],
}

#[derive(Debug)]
pub enum Error {
    READ_OUT_OF_BOUNDS,
    WRITE_OUT_OF_BOUNDS,
    PROGRAM_SIZE_TOO_LARGE,
    FILE_NOT_FOUND,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; MAX_MEMORY_SIZE_BYTES],
        }
    }

    fn read_byte(&self, addr: u16) -> Result<u8, Error> {
        let index = addr as usize;
        if index > MAX_MEMORY_SIZE_BYTES {
            Err(Error::READ_OUT_OF_BOUNDS)
        } else {
            Ok(self.mem[index])
        }
    }

    pub fn read_inst(&self, addr: u16) -> Result<u16, Error> {
        let byte1 = self.read_byte(addr);
        let byte2 = self.read_byte(addr + 1);

        if byte1.is_ok() && byte2.is_ok() {
            let byte_res: u16 = (((byte1.unwrap() as u16) << 8) | (byte2.unwrap() as u16)) as u16;
            Ok(byte_res)
        } else {
            if byte1.is_err() {
                Err(byte1.err().unwrap())
            }else if byte2.is_err() {
                Err(byte2.err().unwrap())
            }else{
                Err(Error::READ_OUT_OF_BOUNDS)
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) -> Result<(), Error> {
        let index = addr as usize;
        if index > MAX_MEMORY_SIZE_BYTES {
            Err(Error::WRITE_OUT_OF_BOUNDS)
        } else {
            self.mem[index] = data;
            Ok(())
        }
    }

    fn load_program_bytes(&mut self, start_addr: u16, bytes: &Vec<u8>) -> Result<(), Error> {
        let start_index = start_addr as usize;
        if start_index > MAX_MEMORY_SIZE_BYTES {
            Err(Error::WRITE_OUT_OF_BOUNDS)
        } else {
            if (start_index + bytes.len() > (MAX_MEMORY_SIZE_BYTES as usize)) {
                Err(Error::PROGRAM_SIZE_TOO_LARGE)
            } else {
                self.mem[start_index..(start_index + bytes.len())]
                    .copy_from_slice(bytes.as_slice());
                Ok(())
            }
        }
    }

    pub fn load_program_from_file(&mut self, start_addr: u16, filename: &str) -> Result<(), Error> {
        if let Ok(mut f) = File::open(&filename) {
            if let Ok(metadata) = std::fs::metadata(&filename) {
                let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
                if let Ok(_) = f.read(&mut buffer) {
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
}
