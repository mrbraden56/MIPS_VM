use serverless::vm::{run, Mips}; // Changed from serverless to crate

#[test]
fn test_addi() {
    // Initialize the MIPS struct with all registers set to 0
    let mut mips = Mips {
        registers: [0; 32],
        memory: [0; 65536],
    };

    // Create the "addi" instruction: addi $8, $0, 3
    // This means $8 = $0 + 3 (i.e., $8 = 0 + 3 => $8 = 3)
    let assembly = "addi $8, $0, 3";

    // Execute the instruction
    run(assembly, &mut mips);

    // Assert that the result in register $8 is 3
    assert_eq!(
        mips.registers[8], 3,
        "Expected $8 to be 3, but got {}",
        mips.registers[8]
    );
}

#[test]
fn test_all_i_type_instructions_except_lui() {
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
