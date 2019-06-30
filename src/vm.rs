use crate::instruction::Opcode;
use std::io::Write;

pub struct VM {
    pub(crate) registers: [i32; 32],
    ip: usize,
    pub(crate) code: Vec<u8>,

    remainder: u32,

    cmp_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            ip: 0,
            code: vec![],
            remainder: 0,
            cmp_flag: false,
        }
    }
    pub fn add_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }
    pub fn set_code(&mut self, vec: &Vec<u8>) {
        self.code = vec.clone();
    }

    pub fn run(&mut self) {
        let mut is_done = false;

        while !is_done {
            is_done = self.exec_instruction();
        }
    }
    pub fn run_once(&mut self) {
        self.exec_instruction();
    }
    fn exec_instruction(&mut self) -> bool {
        // if the instruction pointer exceeds the length of bytecode something is wrong
        if self.ip >= self.code.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::NOP => {
                return false;
            }
            Opcode::HALT => {
                return true;
            }
            Opcode::LOAD => {
                let reg = self.next_8() as usize;
                let val = self.next_16();
                self.registers[reg] = val as i32;
            }
            Opcode::ADD => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.registers[self.next_8() as usize] = self.registers[reg0] + self.registers[reg1];
            }
            Opcode::SUB => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.registers[self.next_8() as usize] = self.registers[reg0] - self.registers[reg1];
            }
            Opcode::MUL => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.registers[self.next_8() as usize] = self.registers[reg0] * self.registers[reg1];
            }
            Opcode::DIV => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.registers[self.next_8() as usize] = self.registers[reg0] / self.registers[reg1];
                self.remainder = (self.registers[reg0] % self.registers[reg1]) as u32;
            }
            Opcode::JMP => {
                let reg = self.next_8() as usize;
                let loc = self.registers[reg];
                self.ip = loc as usize;
            }
            Opcode::JMPF => {
                let loc = self.next_8() as usize;
                self.ip += loc;
            }
            Opcode::JMPB => {
                let loc = self.next_8() as usize;
                self.ip -= loc;
            }
            Opcode::EQ => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.cmp_flag = self.registers[reg0] == self.registers[reg1];
            }
            Opcode::NEQ => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.cmp_flag = self.registers[reg0] != self.registers[reg1];
            }
            Opcode::GT => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.cmp_flag = self.registers[reg0] > self.registers[reg1];
            }
            Opcode::LT => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.cmp_flag = self.registers[reg0] < self.registers[reg1];
            }
            Opcode::GTQ => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.cmp_flag = self.registers[reg0] >= self.registers[reg1];
            }
            Opcode::LTQ => {
                let reg0 = self.next_8() as usize;
                let reg1 = self.next_8() as usize;
                self.cmp_flag = self.registers[reg0] <= self.registers[reg1];
            }
            Opcode::JEQ => {
                let reg = self.next_8() as usize;
                let target = self.registers[reg] as usize;
                if self.cmp_flag {
                    self.ip = target;
                }
            }
            Opcode::JNEQ => {
                let reg = self.next_8() as usize;
                let target = self.registers[reg] as usize;
                if !self.cmp_flag {
                    self.ip = target;
                }
            }
            Opcode::PRT => {
                let reg = self.next_8() as usize;
                let res = self.registers[reg];
                write!(std::io::stdout(), "{}", res).unwrap();
            }
            _ => {
                println!("{} '{}'", format!("{:08x}", self.ip), self.code[self.ip - 1]);
                writeln!(std::io::stderr(), "{}", "Unrecognized opcode found! Terminating!").unwrap();
                return true;
            }
        }
        return false;
    }
    fn next_8(&mut self) -> u8 {
        let result = self.code[self.ip];
        self.ip += 1;
        return result;
    }

    fn next_16(&mut self) -> u16 {
        let result = ((self.code[self.ip] as u16) << 8) | (self.code[self.ip + 1] as u16);
        self.ip += 2;
        return result;
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        if self.ip < self.code.len() {
            let opcode = Opcode::from(self.code[self.ip]);
            self.ip += 1;
            return opcode;
        } else {
            return Opcode::HALT;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_create_vm() {
        let test_vm = VM::new();
        // check that all of the registers are set to zero
        for reg in test_vm.registers.iter() {
            assert_eq!(*reg, 0);
        }
        assert_eq!(test_vm.ip, 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let code = vec![0, 0, 0, 0];
        let mut test_vm = VM::new();
        test_vm.code = code;
        test_vm.run();
        assert_eq!(test_vm.ip, 1);
    }

    #[test]
    fn test_load_opcode() {
        let code = vec![1, 0, 1, 244];
        let mut test_vm = VM::new();
        test_vm.code = code;
        test_vm.run();
        assert_eq!(test_vm.code[0], 1);
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_num() {
        let code = vec![1, 0, 1, 244, 1, 1, 0, 255, 2, 0, 1, 2];
        let mut test_vm = VM::new();
        test_vm.code = code;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 755);
    }

    #[test]
    fn test_jmp() {
        let code = vec![1, 0, 0, 12, 7, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 5];
        let mut test_vm = VM::new();
        test_vm.code = code;
        test_vm.run();
        assert_eq!(test_vm.registers[1], 5);
    }

    #[test]
    fn test_eq() {
        let code = vec![10, 0, 1];
        let mut test_vm: VM = VM::new();
        test_vm.code = code;
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run();
        assert_eq!(test_vm.cmp_flag, true);
    }

    #[test]
    fn test_neq() {
        let code = vec![11, 0, 1];
        let mut test_vm: VM = VM::new();
        test_vm.code = code;
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.run();
        assert_eq!(test_vm.cmp_flag, true);
    }
}
