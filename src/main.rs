mod opcodes;

use bytes::{Bytes, BytesMut, BufMut};


enum InstructionType {
    R,
    I,
    J,
}

enum Register {
    Zero = 0,
    V0 = 2,
    V1 = 3,
    A0 = 4,
    A1 = 5,
    A2 = 6,
    A3 = 7,
    T0 = 8,
    T1 = 9,
    T2 = 10,
    T3 = 11,
    T4 = 12,
    T5 = 13,
    T6 = 14,
    T7 = 15,
    S0 = 16,
    S1 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    T8 = 24,
    T9 = 25,
    Gp = 28,
    Sp = 29,
    Fp = 30,
    Ra = 31,
} // Each register is 32 bits

pub struct Mips32Registers {
    registers: [i32; 32], // Array for the 32 registers
}

pub struct Mips32Memory {
    registers: [i32; 65536],
}

fn execute(opcode: &str, bits: BytesMut) -> () {
    match opcode {
        "addi" => opcodes::addi(bits),
        _ => println!("Unknown opcode: {}", opcode),
    }
}

fn _command_type(value: &str) -> InstructionType {
    if value == "addi" {
        return InstructionType::I;
    } else {
        return InstructionType::I;
    }
}

fn _encode_opcode_name(opcode: &str) -> String {
    let mut opcode_name: String = String::from("addi"); // Make this mutable
    if opcode == "addi" {
        opcode_name = String::from("001000"); // Now this reassignment is legal
    }
    return opcode_name // Remove the `return` keyword and the semicolon
}

fn _encode(value: &str, bit_size: usize) -> String {
    let bitfields: &str = value
        .strip_prefix('$')
        .unwrap_or(value)
        .trim_end_matches(',');
    let mut bitfields_bytes = String::new();
    match bitfields.parse::<i32>() {
        Ok(bitfields_int) => {
            bitfields_bytes = format!("{:0width$b}", bitfields_int, width = bit_size);
        }
        Err(e) => eprintln!("Failed to parse rs: {}", e),
    }

    return bitfields_bytes
}

fn run(assembly: &str) -> () {
    let mut encoding = BytesMut::with_capacity(32);
    // Initialize with zeros
    encoding.resize(32, 0);

    let instructions: Vec<&str> = assembly.split_whitespace().collect();
    match _command_type(instructions[0]) {
        InstructionType::R => {}
        InstructionType::I => {
            let opcode_name = _encode_opcode_name(instructions[0]);
            let rs_bytes = _encode(instructions[1], 5);
            let rt_bytes = _encode(instructions[2], 5);
            let iv_bytes = _encode(instructions[3], 16);
            println!(
                "opcode: {}, rs: {}, rt: {}, immediate: {}",
                opcode_name, rs_bytes, rt_bytes, iv_bytes
            );
            let i_type_encoding_str = format!("{}{}{}{}", opcode_name, rs_bytes, rt_bytes, iv_bytes);
            let bytes = i_type_encoding_str.as_bytes();
            // Clear the existing content and put the new bytes
            encoding.clear();
            encoding.resize(32, 0); 
            let len = std::cmp::min(bytes.len(), 32);
            encoding[..len].copy_from_slice(&bytes[..len]);

            execute(instructions[0], encoding)

        }
        InstructionType::J => {}
    }
}

fn main() {
    let op1: &str = "addi $8, $0, 3";
    let mut commands: Vec<&str> = vec![];
    commands.push(op1);

    for command in commands.iter() {
        run(&command);
    }
}
