# Rust Virtual Machine

This is a simple virtual machine implemented in Rust. It supports a set of instructions for basic arithmetic operations, conditional execution (`IF/ELSE`), reading user input, and variable management.

## Getting Started

- Read the full tutorial: https://medium.com/@luishrsoares/building-a-vm-instruction-set-in-rust-24e3103e1b4d

### Prerequisites

- [Rust](https://www.rust-lang.org/) should be installed on your system.

### Building and Running

1. Clone this repository:

   ```bash
   git clone https://github.com/yourusername/rust-virtual-machine.git `

2.  Navigate to the project directory:

    bashCopy code

    `cd rust-virtual-machine`

3.  Build the project:

    bashCopy code

    `cargo build --release`

4.  Run the virtual machine with an instruction file:

    bashCopy code

    `cargo run path/to/your/instruction_file.txt`

Replace `path/to/your/instruction_file.txt` with the path to your instruction file.

Instruction Set
---------------

The virtual machine supports the following instructions:

-   `PUSH <value>`: Push a value onto the stack.
-   `ADD <operand1> <operand2>`: Add two operands and push the result onto the stack.
-   `SUB <operand1> <operand2>`: Subtract two operands and push the result onto the stack.
-   `MUL <operand1> <operand2>`: Multiply two operands and push the result onto the stack.
-   `DIV <operand1> <operand2>`: Divide two operands and push the result onto the stack.
-   `PRINT`: Print the top value of the stack.
-   `SET <variable> <value>`: Set the value of a variable.
-   `GET <variable>`: Push the value of a variable onto the stack.
-   `Input <variable>`: Read user input and store it in a variable.
-   `IF`: Conditional execution. If the top of the stack is non-zero, execute the following block of instructions.
-   `ELSE`: Specify instructions to execute if the condition in the `IF` block is not met.

### Example Instruction File

plaintextCopy code

`PUSH 10
PRINT
Input y
IF
    GET y
    PRINT
ELSE
    PUSH 0
    PRINT`

This program sets `x` to 10, prints it, reads user input into `y`, and then prints `y` if it's non-zero; otherwise, it prints 0.
