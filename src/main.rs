mod vm;
use vm::{run, Mips};

fn main() {
    let mut mips = Mips {
        registers: [0; 32],
        memory: [0; 65536],
    };

    run("addi $8, $0, 5", &mut mips);
    run("addi $9, $0, 11", &mut mips);
    run("add $10, $8, 9", &mut mips);
}
