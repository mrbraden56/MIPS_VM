mod vm;
use vm::{run, Mips};

fn main() {
    let mut mips = Mips {
        registers: [0; 32],
        memory: [0; 65536],
    };

    //run("addi $8, $0, 3", &mut mips);
    run("addiu $9, $8, 10", &mut mips); // 5 + 10 = 15
}
