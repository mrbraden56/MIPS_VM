use crate::vm::Mips;

pub fn execute_i_type(opcode: &str, rs: i32, rt: i32, iv: i32, mips: &mut Mips) -> () {
    match opcode {
        "addi" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            mips.registers[rt as usize] = mips.registers[rs as usize] + iv;
        }
        "addiu" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            let result = mips.registers[rs as usize] as u32 + iv as u32;
            mips.registers[rt as usize] = result as i32;
        }
        "andi" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            mips.registers[rt as usize] = mips.registers[rs as usize] & iv;
        }
        "ori" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            mips.registers[rt as usize] = mips.registers[rs as usize] | iv;
        }
        "slti" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            mips.registers[rt as usize] = if mips.registers[rs as usize] < iv {
                1
            } else {
                0
            };
        }
        "sltiu" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            if (mips.registers[rs as usize] as u32) < (iv as u32) {
                mips.registers[rt as usize] = 1;
            } else {
                mips.registers[rt as usize] = 0;
            }
        }
        "xori" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            mips.registers[rt as usize] = mips.registers[rs as usize] ^ iv;
        }
        _ => println!("Unknown opcode for i type: {}", opcode),
    }
}

pub fn execute_r_type(opcode: &str, rd: i32, rs: i32, rt: i32, mips: &mut Mips) -> () {
    match opcode {
        "add" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            mips.registers[rd as usize] = mips.registers[rs as usize] + mips.registers[rt as usize];
        }
        "addu" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            let result = mips.registers[rs as usize] + mips.registers[rt as usize];
            mips.registers[rd as usize] = result as i32;

        }
        "sub" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            mips.registers[rd as usize] = mips.registers[rs as usize] - mips.registers[rt as usize];
        }
        "subu" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            let result = mips.registers[rs as usize] - mips.registers[rt as usize];
            mips.registers[rd as usize] = result as i32;

        }
        "and" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            mips.registers[rd as usize] = mips.registers[rs as usize] & mips.registers[rt as usize];
        }
        "or" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            mips.registers[rd as usize] = mips.registers[rs as usize] | mips.registers[rt as usize];
        }
        "xor" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            mips.registers[rd as usize] = mips.registers[rs as usize] ^ mips.registers[rt as usize];
        }
        "nor" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            mips.registers[rd as usize] = !(mips.registers[rs as usize] | mips.registers[rt as usize]);
        }
        "slt" => {
            println!(
                "Executing Opcode name: {}, rd: {}, rs: {}, rt: {}",
                opcode, rd, rs, rt
            );
            mips.registers[rd as usize] = if mips.registers[rs as usize] < mips.registers[rt as usize] {
                1
            } else {
                0
            };
        }
        _ => println!("Unknown opcode for r type: {}", opcode),
    }
}
