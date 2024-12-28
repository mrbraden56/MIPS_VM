use crate::vm::Mips;

pub fn execute_i_type(opcode: &str, rs: i32, rt: i32, iv: i32, mips: &mut Mips) -> () {
    match opcode {
        "addi" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            mips.registers[rs as usize] = mips.registers[rt as usize] + iv;
            assert!(
                mips.registers[rs as usize] == mips.registers[rt as usize] + iv,
                "ADDI operation failed: {} & {} = {}",
                mips.registers[rt as usize],
                iv,
                mips.registers[rs as usize]
            );
        }
        "addiu" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );

            // Perform the addition, treating the values as unsigned
            let result = mips.registers[rs as usize] as u32 + iv as u32;

            // Store the result back in the target register (treat as unsigned)
            mips.registers[rt as usize] = result as i32;

            // Optionally, print the result to verify
            println!(
                "Result of {} + {} (unsigned): {}",
                mips.registers[rs as usize], iv, mips.registers[rt as usize]
            );
        }

        "andi" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            mips.registers[rs as usize] = mips.registers[rt as usize] & iv;
            assert!(
                mips.registers[rs as usize] == (mips.registers[rt as usize] & iv),
                "ANDI operation failed: {} & {} = {}",
                mips.registers[rt as usize],
                iv,
                mips.registers[rs as usize]
            );
        }

        "ori" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );

            mips.registers[rs as usize] = mips.registers[rt as usize] | iv;

            assert!(
                mips.registers[rs as usize] == mips.registers[rt as usize] | iv,
                "rs != rt | iv where rs is {} and rt | iv is {} | {}",
                mips.registers[rs as usize],
                mips.registers[rt as usize],
                iv
            );
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

            println!(
                "Set rt to {} because rs {} is {} than iv {}",
                mips.registers[rt as usize],
                mips.registers[rs as usize],
                if mips.registers[rs as usize] < iv {
                    "less"
                } else {
                    "not less"
                },
                iv
            );
        }

        "sltiu" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );

            // Compare the values as unsigned integers by casting both to u32
            if (mips.registers[rs as usize] as u32) < (iv as u32) {
                mips.registers[rt as usize] = 1; // Set rt to 1 if condition is true
            } else {
                mips.registers[rt as usize] = 0; // Set rt to 0 if condition is false
            }

            // Optionally, print the result to verify
            println!(
                "Result of comparison (unsigned): {}",
                mips.registers[rt as usize]
            );
        }

        "xori" => {
            println!(
                "Executing Opcode name: {}, rs: {}, rt: {}, Immediate value: {}",
                opcode, rs, rt, iv
            );
            // Perform XOR between the value in register rs and the immediate value iv
            mips.registers[rt as usize] = mips.registers[rs as usize] ^ iv;

            println!(
                "Set rt to {} because rs ({} XOR iv {})",
                mips.registers[rt as usize], mips.registers[rs as usize], iv
            );
        }

        "lui" => {
            println!(
                "Executing Opcode name: {}, rt: {}, Immediate value: {}",
                opcode, rt, iv
            );
            // Shift the immediate value (iv) to the upper 16 bits and store it in rt
            mips.registers[rt as usize] = iv << 16;

            println!(
                "Set rt to {} with upper 16 bits of iv {}",
                mips.registers[rt as usize], iv
            );
        }

        _ => println!("Unknown opcode: {}", opcode),
    }
}
