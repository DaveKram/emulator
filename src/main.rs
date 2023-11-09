mod cpu;
mod debugger;
mod memory;

fn main() {
    
    //Startup print
    println!("======================================");
    println!("6502 Emulator");
    println!("======================================");


    //Create the memory
    let mut mem = memory::Memory::new();

    //Create the cpu
    let mut cpu = cpu::CPU::new();

    //Create the debugger
    let mut debugger = debugger::Debugger::new(true);

    //Load program
    if let Ok(_) = mem.load_program_from_file(0, "programs/fast-multiply-by-ten.bin", true) {
        //Continue to execute instructions untilwe need to halt
        while !cpu.check_halt() {
            //Check to see if the debugger is enabled/disabled
            if debugger.is_enabled() {
                debugger.execute_next_user_action(&mut cpu, &mut mem);
            } else {
                cpu.step(&mut mem);
            }
        }
    }
}
