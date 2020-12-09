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

    let instructions = opcodes
        .into_iter()
        .map(Instruction::new)
        .collect::<Vec<_>>();

    let mut patch_start_index = 0;

    loop {
        let mut patched_instructions = Vec::with_capacity(instructions.len());

        let mut patched = false;
        for (index, instruction) in instructions.iter().enumerate() {
            if index < patch_start_index {
                patched_instructions.push(instruction.clone());
                continue;
            }

            let patched_instruction = if !patched {
                Instruction::new(match instruction.opcode {
                    Opcode::Jmp(value) => {
                        patched = true;
                        patch_start_index = index + 1;
                        Opcode::Nop(value)
                    }
                    Opcode::Nop(value) => {
                        patched = true;
                        patch_start_index = index + 1;
                        Opcode::Jmp(value)
                    }
                    opcode => opcode,
                })
            } else {
                instruction.clone()
            };

            patched_instructions.push(patched_instruction);
        }

        let state = execute_instruction_set(patched_instructions);

        if state.success {
            println!(
                "We've executed instruction set successfully, accumulator value is: {}",
                state.accumulator
            );
            break;
        }
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
