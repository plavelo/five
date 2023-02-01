use clap::Parser;
use five::{
    emulator::{
        cpu::{csr::Csr, Cpu},
        Emulator,
    },
    isa::csr::user_level::CYCLE,
};
use std::fs::File;
use std::io::Result;

#[derive(Parser)]
struct Opts {
    #[clap(short, long, default_value_t = 0)]
    timeout: u64,
    #[clap(short, long, action)]
    debug: bool,
    input: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let input = opts.input;
    let file = File::open(&input)?;
    let mut emulator = Emulator::default();
    emulator.load(file)?;
    let terminator = Some(|cpu: &Cpu| {
        if opts.timeout > 0 && cpu.csr.read(CYCLE) > opts.timeout {
            println!("timeout");
            return Some(0);
        }
        let value = cpu.bus.load64(0x80001000);
        if value == 1 {
            Some(1)
        } else {
            None
        }
    });
    let result = emulator.run(opts.debug, terminator);
    if result == 1 {
        println!("PASS: {}", input);
    } else {
        println!("FAIL({}): {}", result, input);
    }
    Ok(())
}
