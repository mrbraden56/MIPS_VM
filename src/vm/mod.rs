mod opcodes;

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

pub struct Mips {
    pub registers: [i32; 32],
    pub memory: [i32; 65536],
}

fn _instruction_type(value: &str) -> InstructionType {
    if value == "addi" {
        return InstructionType::I;
    } else {
        return InstructionType::I;
    }
}

fn _encode(value: &str) -> i32 {
    let parsed_string = value
        .strip_prefix('$')
        .unwrap_or(value)
        .trim_end_matches(',');
    parsed_string
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Failed to parse '{}' into i32", parsed_string))
}

pub fn run(assembly: &str, mips: &mut Mips) -> () {
    let instructions: Vec<&str> = assembly.split_whitespace().collect();
    match _instruction_type(instructions[0]) {
        InstructionType::R => {}
        InstructionType::I => {
            let opcode = instructions[0];
            let rs = _encode(instructions[1]);
            let rt = _encode(instructions[2]);
            let iv = _encode(instructions[3]);
            opcodes::execute_i_type(opcode, rs, rt, iv, mips)
        }
        InstructionType::J => {}
    }
}
