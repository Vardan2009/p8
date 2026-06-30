mod emu;
mod fileop;
mod inst;
mod opcode;
mod reg;

use std::io;
use std::io::Write;

fn step(cmd: &Vec<&str>, cpu_state: &mut emu::CPUState) {
    let mut step_count: i8 = 1;

    if cmd.len() == 2 {
        let parse_result = cmd[1].parse::<i8>();

        step_count = match parse_result {
            Ok(step_count) => step_count,
            Err(_error) => {
                print!("Failed to parse argument\n");
                return;
            }
        };
    } else if cmd.len() != 1 {
        print!("Invalid arguments: usage: step [num, default 1]\n");
        return;
    }

    if step_count <= 0 {
        print!("Can't take <= 0 steps\n");
        return;
    }

    for _ in 0..step_count {
        emu::step(cpu_state);
    }
}

fn run(cpu_state: &mut emu::CPUState) {
    while (cpu_state.progmem[cpu_state.pc as usize] & 0xF000) != 0xF000 {
        emu::step(cpu_state);
    }
}

fn view_disasm_nearby(cpu_state: &emu::CPUState) {
    let pc = cpu_state.pc as usize;
    let start = pc.saturating_sub(3);
    let end = pc.saturating_add(3);

    for (index, inst) in cpu_state.progmem.iter().enumerate() {
        if index < start || index > end {
            continue;
        }

        let decoded = inst::decode(index as u8, *inst);

        if index as u8 == cpu_state.pc {
            print!("> ");
        }

        inst::print(&decoded);
    }
}

fn print_state(cpu_state: &emu::CPUState) {
    print!("======= CPU STATE =======\n\n");
    print!("======= REGISTERS =======\n");
    print!(" - R0: {:02x}\n", cpu_state.r0);
    print!(" - R1: {:02x}\n", cpu_state.r1);
    print!(" - R2: {:02x}\n", cpu_state.r2);
    print!(" - R3: {:02x}\n", cpu_state.r3);
    print!(" - R4: {:02x}\n", cpu_state.r4);
    print!(" - R5: {:02x}\n", cpu_state.r5);
    print!(" - IA: {:02x}\n", cpu_state.ia);
    print!(" - SP: {:02x}\n", cpu_state.sp);
    print!(" - PC: {:02x}\n\n", cpu_state.pc);

    print!(" - Z:  {}\n", if cpu_state.zflag { "SET" } else { "UNSET" });
    print!(
        " - N:  {}\n\n",
        if cpu_state.nflag { "SET" } else { "UNSET" }
    );

    print!("\n===== PROGRAM STATE =====\n");

    view_disasm_nearby(cpu_state);

    print!("\n=========================\n");
}

fn view_disasm(cpu_state: &emu::CPUState) {
    for (index, inst) in cpu_state.progmem.iter().enumerate() {
        let decoded = inst::decode(index as u8, *inst);

        if index as u8 == cpu_state.pc {
            print!("> ");
        }

        inst::print(&decoded);
    }
}

fn unknown_command(cmd: &Vec<&str>) {
    print!("Unknown command `{}`\n", cmd[0]);
}

fn load_file(cmd: &Vec<&str>, cpu_state: &mut emu::CPUState) {
    if cmd.len() != 2 {
        print!("Invalid arguments: usage: <l|load> <filepath>\n");
        return;
    }

    let _ = fileop::read_hex_file(cmd[1], &mut cpu_state.progmem);
}

fn load_rom(cmd: &Vec<&str>, cpu_state: &mut emu::CPUState) {
    if cmd.len() != 2 {
        print!("Invalid arguments: usage: <lr|loadrom> <filepath>\n");
        return;
    }

    let _ = fileop::read_hex_file_u8(cmd[1], &mut cpu_state.rom);
}

fn memdump(cpu_state: &emu::CPUState) {
    for (addr, val) in cpu_state.ram.iter().enumerate() {
        print!("{:02x}: {:02x}\n", addr as u8, val);
    }
    print!("=== ROM AREA\n");
    for (addr, val) in cpu_state.rom.iter().enumerate() {
        print!("{:02x}: {:02x}\n", (addr + 0x80) as u8, val);
    }
}

fn jump(cmd: &Vec<&str>, cpu_state: &mut emu::CPUState) {
    if cmd.len() != 2 {
        print!("Invalid arguments: usage: <j|jump> <new pc>\n");
        return;
    }

    let parse_result = cmd[1].parse::<u8>();

    let newpc = match parse_result {
        Ok(newpc) => newpc,
        Err(_error) => {
            print!("Failed to parse argument\n");
            return;
        }
    };

    cpu_state.pc = newpc;
}

fn script(cmd: &Vec<&str>, cpu_state: &mut emu::CPUState) {
    if cmd.len() != 2 {
        print!("Invalid arguments: usage: <script|sc> <filepath>\n");
        return;
    }

    let result = fileop::read_lines(cmd[1]);

    let lines = match result {
        Ok(lines) => lines,
        Err(_) => {
            print!("Failed to read file `{}`\n", cmd[1]);
            return;
        }
    };

    for line in lines {
        execute_command(&line, cpu_state);
    }
}

fn execute_command(input: &String, cpu_state: &mut emu::CPUState) {
    if input.is_empty() {
        return;
    }

    let trimmed_input = match input.trim().split_once('#') {
        Some((before, _)) => before.trim_end(),
        None => input,
    };

    if trimmed_input.is_empty() {
        return;
    }

    let cmd: Vec<&str> = trimmed_input.split_whitespace().collect();

    if cmd.len() < 1 {
        return;
    }

    match cmd[0] {
        "status" | "st" => print_state(cpu_state),
        "step" | "s" => step(&cmd, cpu_state),
        "run" | "r" => run(cpu_state),
        "disasm" | "d" => view_disasm(cpu_state),
        "load" | "l" => load_file(&cmd, cpu_state),
        "loadrom" | "lr" => load_rom(&cmd, cpu_state),
        "memdump" | "m" => memdump(cpu_state),
        "jump" | "j" => jump(&cmd, cpu_state),
        "reset" | "re" => emu::reset(cpu_state),
        "script" | "sc" => script(&cmd, cpu_state),
        _ => unknown_command(&cmd),
    }
}

fn main() {
    let mut cpu_state = emu::CPUState {
        r0: 0,
        r1: 0,
        r2: 0,
        r3: 0,
        r4: 0,
        r5: 0,
        ia: 0,
        sp: 0,
        pc: 0,
        zflag: false,
        nflag: false,
        progmem: [0; 256],
        ram: [0; 128],
        rom: [0; 128],
    };

    loop {
        print!("p8emu [{:02x}]> ", cpu_state.pc);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        execute_command(&input, &mut cpu_state);
    }
}
