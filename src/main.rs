use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, process};

#[derive(Clone)]
enum Operand {
    Value(i32),
    Var(String),
}

#[derive(Clone)]
enum Instruction {
    Push(i32),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Print,
    Set(String, i32),
    Get(String),
    Input(String),
    If(Vec<Instruction>, Vec<Instruction>),
    Else(Vec<Instruction>),
}

struct VM {
    stack: Vec<i32>,
    vars: HashMap<String, i32>,
}

impl VM {
    fn new() -> VM {
        VM {
            stack: Vec::new(),
            vars: HashMap::new(),
        }
    }

    fn get_operand_value(&self, operand: &Operand) -> i32 {
        match operand {
            Operand::Value(val) => *val,
            Operand::Var(var_name) => *self.vars.get(var_name)
                .expect("Variable not found"),
        }
    }

    fn run(&mut self, program: Vec<Instruction>, path: &str) {
        let mut pc = 0; // Program counter
        while pc < program.len() {
            match &program[pc] {
                //PUSH
                Instruction::Push(val) => self.stack.push(*val),

                //ADDITION
                Instruction::Add(op1, op2) => {
                    let val1 = self.get_operand_value(op1);
                    let val2 = self.get_operand_value(op2);
                    self.stack.push(val1 + val2);
                },

                //SUBSTRACTION
                Instruction::Sub(op1, op2) => {
                    let val1 = self.get_operand_value(op1);
                    let val2 = self.get_operand_value(op2);
                    self.stack.push(val1 - val2);
                },

                //MULTIPLICATION
                Instruction::Mul(op1, op2) => {
                    let val1 = self.get_operand_value(op1);
                    let val2 = self.get_operand_value(op2);
                    self.stack.push(val1 * val2);
                },

                //DIVISION
                Instruction::Div(op1, op2) => {
                    let val1 = self.get_operand_value(op1);
                    let val2 = self.get_operand_value(op2);
                    if val2 == 0 {
                        panic!("Division by zero");
                    }
                    self.stack.push(val1 / val2);
                },

                //PRINT
                Instruction::Print => {
                    if let Some(top) = self.stack.last() {
                        println!("{}", top);
                    } else {
                        println!("Stack is empty");
                    }
                },

                //SET VARIABLE
                Instruction::Set(var_name, value) => {
                    self.vars.insert(var_name.clone(), *value);
                },

                //GET VARIABLE
                Instruction::Get(var_name) => {
                    if let Some(&value) = self.vars.get(var_name) {
                        self.stack.push(value);
                    } else {
                        panic!("Undefined variable: {}", var_name);
                    }
                },

                //GET USER INPUT from the command line
                Instruction::Input(var_name) => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    let value = input.trim().parse::<i32>().expect("Invalid input");
                    self.vars.insert(var_name.clone(), value);
                },

                //PROCESS IF instructions
                Instruction::If(if_block, else_block) => {
                    if let Some(top) = self.stack.last() {
                        if *top != 0 {
                            self.run(if_block.to_vec(), path); // IF the value at the stack is > 0, execute the IF instruction
                        } else if !else_block.is_empty() { // If the value at the stack = 0, execute the else
                            if let Ok(file) = File::open(path) {
                                let reader = io::BufReader::new(file);
                                let mut else_block_clone = else_block.clone(); // Clone the else_block
                                let mut else_block_reader = reader.lines();

                                for next_line in &mut else_block_reader {
                                    if let Ok(next_line) = next_line {
                                        else_block_clone.extend(parse_instruction(&next_line));
                                    }
                                }
                                self.run(else_block_clone, path); // Pass the cloned else_block
                            } else {
                                panic!("Failed to open file: {}", path);
                            }
                        }
                    } else {
                        panic!("Stack is empty");
                    }
                },

                //Process the ELSE block
                Instruction::Else(else_block) => {
                    // This is only executed if the 'if' condition was not met,
                    // so we don't need to check the stack again.
                    self.run(else_block.to_vec(), path); // Pass path as an argument
                },
            }
            pc += 1;
        }
    }

    fn load_program(reader: &mut io::BufReader<File>) -> io::Result<Vec<Instruction>> {
        let mut program = Vec::new();

        // Read all lines into a vector
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        // Temporary storage for IF/ELSE blocks
        let mut if_block = Vec::new();
        let mut else_block = Vec::new();
        let mut in_if_block = false;
        let mut in_else_block = false;

        for line in lines.iter() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            // Handle the start of an IF block
            if parts.get(0) == Some(&"IF") {
                in_if_block = true;
                in_else_block = false;
                continue;
            }

            // Handle the start of an ELSE block
            if parts.get(0) == Some(&"ELSE") {
                in_else_block = true;
                in_if_block = false;
                continue;
            }

            // Check if currently inside an IF or ELSE block
            if in_if_block || in_else_block {
                let block = if in_if_block { &mut if_block } else { &mut else_block };

                // Add instruction to the current block
                block.extend(parse_instruction(line));

                // Check for the end of the block
                if parts.get(0) == Some(&"ENDIF") {
                    if in_if_block {
                        program.push(Instruction::If(if_block.clone(), else_block.clone()));
                    } else {
                        program.push(Instruction::Else(else_block.clone()));
                    }
                    if_block.clear();
                    else_block.clear();
                    in_if_block = false;
                    in_else_block = false;
                }

                continue;
            }

            // Parse other instructions
            let instruction = parse_instruction(line);
            program.extend(instruction);
        }

        Ok(program)
    }
}

fn parse_operand(op_str: &str) -> Operand {
    if let Ok(val) = op_str.parse::<i32>() {
        Operand::Value(val)
    } else {
        Operand::Var(op_str.to_string())
    }
}

fn extract_var_name(operand: &str) -> &str {
    operand.trim_start_matches("Var(\"").trim_end_matches("\")")
}

fn parse_instruction(line: &str) -> Vec<Instruction> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        ["PUSH", num] => vec![Instruction::Push(num.parse::<i32>().expect("Invalid number"))],
        ["ADD", op1, op2] => {
            let operand1 = parse_operand(extract_var_name(op1));
            let operand2 = parse_operand(extract_var_name(op2));
            vec![Instruction::Add(operand1, operand2)]
        },
        ["SUB", op1, op2] => {
            let operand1 = parse_operand(extract_var_name(op1));
            let operand2 = parse_operand(extract_var_name(op2));
            vec![Instruction::Sub(operand1, operand2)]
        },
        ["MUL", op1, op2] => {
            let operand1 = parse_operand(extract_var_name(op1));
            let operand2 = parse_operand(extract_var_name(op2));
            vec![Instruction::Mul(operand1, operand2)]
        },
        ["DIV", op1, op2] => {
            let operand1 = parse_operand(extract_var_name(op1));
            let operand2 = parse_operand(extract_var_name(op2));
            vec![Instruction::Div(operand1, operand2)]
        },
        ["PRINT"] => vec![Instruction::Print],
        ["SET", var_name, value] => {
            let value = value.parse::<i32>().expect("Invalid number");
            vec![Instruction::Set(var_name.to_string(), value)]
        },
        ["GET", var_name] => vec![Instruction::Get(var_name.to_string())],
        ["Input", var_name] => vec![Instruction::Input(var_name.to_string())],
        _ => vec![],
    }
}

// Function to create a BufReader and call VM::load_program
fn load_program_and_run(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(Box::new(e)); // Return an error
        }
    };
    let mut reader = io::BufReader::new(file);

    // Create a VM instance
    let mut vm = VM::new();

    // Load and run the program
    match VM::load_program(&mut reader) {
        Ok(program) => {
            vm.run(program, file_path); // Just call run without expecting a Result
            // Handle any other necessary logic here if needed
        }
        Err(e) => {
            eprintln!("Failed to load program: {}", e);
            return Err(Box::new(e)); // Return an error
        }
    }

    Ok(()) // Return Ok to indicate success
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <program_file.rm>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    match load_program_and_run(file_path) {
        Ok(_) => {
            println!("Program executed successfully.");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
