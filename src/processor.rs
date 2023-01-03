const end_of_chip8: u16 = 0xfff;
const start_of_eti: u16 = 0x600;
const start_of_programs: u16 = 0x200;
const start_of_chip8: u16 = 0x000;


pub struct CPU //all the main stuff
//acording to http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
{
    memory_range:   [u16; 4096],        //ram
    stack:          [u16; 16],
    index:          u16,
    registers:      [u8; 16],
    sp:             u8,         //stack pointer
    pc:             u16,        //program counter
    delay:          u8,         //delay timer
    sound:          u8,         //sound timer
    // wip: keypad:         
    // wip: display:        
}

impl CPU {
    pub fn clean() //brand new cpu
    {
        CPU
        {
            memory_range:   [0,4096],
            stack:          [0,16],
            index:          0,
            registers:      [0,16],
            sp:             0,
            pc:             0,
            delay:          0,
            sound:          0,
            // wip: keypad:         
            // wip: display:  
        }
    }
}
