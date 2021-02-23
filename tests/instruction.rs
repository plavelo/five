use five::emulator::Emulator;
use std::fs::File;
use std::path::PathBuf;

fn run(name: &str) -> bool {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("riscv-tests");
    path.push("isa");
    path.push(name);
    path.set_extension("bin");
    let file = File::open(&path.as_path());
    let mut emulator = Emulator::default();
    if let Ok(f) = file {
        let _ = emulator.load(f);
        emulator.run() == 1
    } else {
        false
    }
}

#[test]
fn rv32uip_ok() {
    assert!(run("rv32ui-p-add"), "{}", "rv32ui-p-add");
    assert!(run("rv32ui-p-addi"), "{}", "rv32ui-p-addi");
    assert!(run("rv32ui-p-and"), "{}", "rv32ui-p-and");
    assert!(run("rv32ui-p-andi"), "{}", "rv32ui-p-andi");
    assert!(run("rv32ui-p-auipc"), "{}", "rv32ui-p-auipc");
    assert!(run("rv32ui-p-beq"), "{}", "rv32ui-p-beq");
    assert!(run("rv32ui-p-bge"), "{}", "rv32ui-p-bge");
    assert!(run("rv32ui-p-bgeu"), "{}", "rv32ui-p-bgeu");
    assert!(run("rv32ui-p-blt"), "{}", "rv32ui-p-blt");
    assert!(run("rv32ui-p-bltu"), "{}", "rv32ui-p-bltu");
    assert!(run("rv32ui-p-bne"), "{}", "rv32ui-p-bne");
    assert!(run("rv32ui-p-fence_i"), "{}", "rv32ui-p-fence_i");
    assert!(run("rv32ui-p-jal"), "{}", "rv32ui-p-jal");
    assert!(run("rv32ui-p-jalr"), "{}", "rv32ui-p-jalr");
    assert!(run("rv32ui-p-lb"), "{}", "rv32ui-p-lb");
    assert!(run("rv32ui-p-lbu"), "{}", "rv32ui-p-lbu");
    assert!(run("rv32ui-p-lh"), "{}", "rv32ui-p-lh");
    assert!(run("rv32ui-p-lhu"), "{}", "rv32ui-p-lhu");
    assert!(run("rv32ui-p-lui"), "{}", "rv32ui-p-lui");
    assert!(run("rv32ui-p-lw"), "{}", "rv32ui-p-lw");
    assert!(run("rv32ui-p-or"), "{}", "rv32ui-p-or");
    assert!(run("rv32ui-p-ori"), "{}", "rv32ui-p-ori");
    assert!(run("rv32ui-p-sb"), "{}", "rv32ui-p-sb");
    assert!(run("rv32ui-p-sh"), "{}", "rv32ui-p-sh");
    assert!(run("rv32ui-p-simple"), "{}", "rv32ui-p-simple");
    assert!(run("rv32ui-p-sll"), "{}", "rv32ui-p-sll");
    assert!(run("rv32ui-p-slli"), "{}", "rv32ui-p-slli");
    assert!(run("rv32ui-p-slt"), "{}", "rv32ui-p-slt");
    assert!(run("rv32ui-p-slti"), "{}", "rv32ui-p-slti");
    assert!(run("rv32ui-p-sltiu"), "{}", "rv32ui-p-sltiu");
    assert!(run("rv32ui-p-sltu"), "{}", "rv32ui-p-sltu");
    assert!(run("rv32ui-p-sra"), "{}", "rv32ui-p-sra");
    assert!(run("rv32ui-p-srai"), "{}", "rv32ui-p-srai");
    assert!(run("rv32ui-p-srl"), "{}", "rv32ui-p-srl");
    assert!(run("rv32ui-p-srli"), "{}", "rv32ui-p-srli");
    assert!(run("rv32ui-p-sub"), "{}", "rv32ui-p-sub");
    assert!(run("rv32ui-p-sw"), "{}", "rv32ui-p-sw");
    assert!(run("rv32ui-p-xor"), "{}", "rv32ui-p-xor");
    assert!(run("rv32ui-p-xori"), "{}", "rv32ui-p-xori");
}
