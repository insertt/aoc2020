use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_data = fs::read_to_string("input/day08/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let mut opcodes: Vec<Opcode> = Vec::new();

    for line in input_data.lines() {
        let (label, value) = scan_fmt::scan_fmt!(line, "{} {}", String, i32)?;

        let opcode = match label.as_str() {
            "nop" => Opcode::Nop(value),
            "acc" => Opcode::Acc(value),
            "jmp" => Opcode::Jmp(value),
            _ => panic!("unknown opcode"),
        };

        opcodes.push(opcode);
    }

    let instructions = opcodes.into_iter().map(Instruction::new).collect();
    let state = execute_instruction_set(instructions);

    if !state.success {
        println!(
            "We've got a loop, accumulator value is: {:?}",
            state.accumulator
        );
    }

    Ok(())
}

struct ExecutionState {
    inst_index: usize,
    accumulator: i32,
    success: bool,
}

fn execute_instruction_set(mut instruction_set: Vec<Instruction>) -> ExecutionState {
    let mut state = ExecutionState {
        inst_index: 0,
        accumulator: 0,
        success: true,
    };

    loop {
        let instruction = &mut instruction_set[state.inst_index];
        execute_instruction(instruction, &mut state);

        if state.inst_index >= instruction_set.len() {
            return state;
        }

        let next = &instruction_set[state.inst_index];

        if next.executed {
            state.success = false;
            return state;
        }
    }
}

fn execute_instruction(instruction: &mut Instruction, state: &mut ExecutionState) {
    match instruction.opcode {
        Opcode::Acc(value) => {
            state.accumulator += value;
            state.inst_index += 1;
        }
        Opcode::Jmp(value) => {
            state.inst_index = (state.inst_index as i32 + value) as usize;
        }
        Opcode::Nop(_) => {
            state.inst_index += 1;
        }
    }

    instruction.executed = true;
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    executed: bool,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self {
            opcode,
            executed: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}
