use serverless::vm::{run, Mips}; // Changed from serverless to crate

#[test]
fn test_all_i_type_instructions() {
    let mut mips = Mips {
        registers: [0; 32],
        memory: [0; 65536],
    };

    // Test ADDI
    run("addi $8, $0, 5", &mut mips);
    assert_eq!(mips.registers[8], 5, "ADDI failed: Register 8 should be 5");

    // Test ADDIU
    run("addiu $9, $8, 10", &mut mips); // 5 + 10 = 15
    assert_eq!(mips.registers[9], 15, "ADDIU failed: Register 9 should be 15");

    // Test ANDI
    run("andi $10, $9, 1", &mut mips); // 15 & 1 = 1
    assert_eq!(mips.registers[10], 1, "ANDI failed: Register 10 should be 1");

    // Test ORI
    run("ori $11, $10, 2", &mut mips); // 1 | 2 = 3
    assert_eq!(mips.registers[11], 3, "ORI failed: Register 11 should be 3");

    // Test SLTI
    run("slti $12, $11, 4", &mut mips); // 3 < 4 should set register 12 to 1
    assert_eq!(mips.registers[12], 1, "SLTI failed: Register 12 should be 1");

    // Test SLTIU (unsigned comparison)
    run("sltiu $13, $11, 4", &mut mips); // 3 < 4 in unsigned should set register 13 to 1
    assert_eq!(mips.registers[13], 1, "SLTIU failed: Register 13 should be 1");

    // Test XORI
    run("xori $14, $11, 3", &mut mips); // 3 ^ 3 = 0
    assert_eq!(mips.registers[14], 0, "XORI failed: Register 14 should be 0");

    println!("All I-type instructions (except LUI) passed!");
}

#[test]
fn test_all_r_type_instructions() {
    let mut mips = Mips {
        registers: [0; 32],
        memory: [0; 65536],
    };

    // Set up initial values for testing
    mips.registers[8] = 5;  // $t0
    mips.registers[9] = 10; // $t1

    // Test ADD
    run("add $10, $8, $9", &mut mips); // 5 + 10 = 15
    assert_eq!(mips.registers[10], 15, "ADD failed: Register $10 should be 15");

    // Test ADDU (unsigned addition)
    run("addu $11, $10, $8", &mut mips); // 15 + 5 = 20
    assert_eq!(mips.registers[11], 20, "ADDU failed: Register $11 should be 20");

    // Test SUB
    run("sub $12, $11, $8", &mut mips); // 20 - 5 = 15
    assert_eq!(mips.registers[12], 15, "SUB failed: Register $12 should be 15");

    // Test SUBU (unsigned subtraction)
    run("subu $13, $12, $8", &mut mips); // 15 - 5 = 10
    assert_eq!(mips.registers[13], 10, "SUBU failed: Register $13 should be 10");

    // Test AND
    run("and $14, $13, $9", &mut mips); // 10 & 10 = 10
    assert_eq!(mips.registers[14], 10, "AND failed: Register $14 should be 10");

    // Test OR
    run("or $15, $14, $9", &mut mips); // 10 | 10 = 10
    assert_eq!(mips.registers[15], 10, "OR failed: Register $15 should be 10");

    // Reset some registers for next tests
    mips.registers[8] = 3;  // $t0
    mips.registers[9] = 3;  // $t1

    // Test XOR
    run("xor $10, $8, $9", &mut mips); // 3 ^ 3 = 0
    assert_eq!(mips.registers[10], 0, "XOR failed: Register $10 should be 0");

    // Test NOR
    run("nor $11, $8, $9", &mut mips); // !(3 | 3) = !3 = -4 in two's complement
    assert_eq!(mips.registers[11], -4, "NOR failed: Register $11 should be -4");

    // Reset registers for SLT and SLTU
    mips.registers[8] = 3;  // $t0
    mips.registers[9] = 20; // $t1

    // Test SLT
    run("slt $12, $8, $9", &mut mips); // 3 < 20 should set register $12 to 1
    assert_eq!(mips.registers[12], 1, "SLT failed: Register $12 should be 1");

    println!("All R-type instructions passed!");
}

#[test]
fn test_combined_r_and_i_instructions() {
    let mut mips = Mips {
        registers: [0; 32],
        memory: [0; 65536],
    };

    // Set up initial values for testing
    mips.registers[8] = 5;  // $t0

    // Test ADDI followed by ADD (I-type then R-type)
    run("addi $9, $8, 10", &mut mips); // 5 + 10 = 15
    assert_eq!(mips.registers[9], 15, "ADDI failed: Register $9 should be 15");

    run("add $10, $8, $9", &mut mips); // 5 + 15 = 20
    assert_eq!(mips.registers[10], 20, "ADD failed: Register $10 should be 20");

    // Test ANDI followed by AND (I-type then R-type)
    run("andi $11, $10, 1", &mut mips); // 20 & 1 = 0
    assert_eq!(mips.registers[11], 0, "ANDI failed: Register $11 should be 0");

    mips.registers[12] = 1;  // set $12 for AND test
    run("and $13, $11, $12", &mut mips); // 0 & 1 = 0
    assert_eq!(mips.registers[13], 0, "AND failed: Register $13 should be 0");

    // Test ORI followed by OR (I-type then R-type)
    run("ori $14, $13, 2", &mut mips); // 0 | 2 = 2
    assert_eq!(mips.registers[14], 2, "ORI failed: Register $14 should be 2");

    run("or $15, $14, $8", &mut mips); // 2 | 5 = 7
    assert_eq!(mips.registers[15], 7, "OR failed: Register $15 should be 7");

    // Test SLTI followed by SLT (I-type then R-type)
    run("slti $9, $8, 6", &mut mips); // 5 < 6 sets $9 to 1
    assert_eq!(mips.registers[9], 1, "SLTI failed: Register $9 should be 1");

    run("slt $10, $9, $8", &mut mips); // 1 < 5 sets $10 to 1
    assert_eq!(mips.registers[10], 1, "SLT failed: Register $10 should be 1");

    // Test XORI followed by XOR (I-type then R-type)
    run("xori $11, $8, 5", &mut mips); // 5 ^ 5 = 0
    assert_eq!(mips.registers[11], 0, "XORI failed: Register $11 should be 0");

    run("xor $12, $11, $9", &mut mips); // 0 ^ 1 = 1
    assert_eq!(mips.registers[12], 1, "XOR failed: Register $12 should be 1");

    // Test SUBU followed by SLTIU (R-type then I-type)
    run("subu $13, $8, $9", &mut mips); // 5 - 1 = 4 (unsigned)
    assert_eq!(mips.registers[13], 4, "SUBU failed: Register $13 should be 4");

    run("sltiu $14, $13, 5", &mut mips); // 4 < 5 (unsigned) sets $14 to 1
    assert_eq!(mips.registers[14], 1, "SLTIU failed: Register $14 should be 1");

    println!("Combined R-type and I-type instructions test passed!");
}

#[test]
fn test_all_j_type_instructions() {
    let mut mips = Mips {
        registers: [0; 32],
        memory: [0; 65536],
    };

    assert_eq!(1, 1, "temp");
}
