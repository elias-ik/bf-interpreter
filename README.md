# Brainfuck Interpreter in Rust

This program is a Brainfuck interpreter implemented in Rust. Brainfuck is an esoteric programming language with only eight instructions, making it very simple yet challenging to program in.

The program reads in a Brainfuck file, parses the instructions, and then executes them.

## Usage

To use this program, you need to have Rust installed on your computer. You can download it from https://www.rust-lang.org/tools/install.

Once Rust is installed, clone this repository, navigate to the directory containing the source code, and run the following command:

```rust
cargo run <filename.bf>
```
Replace `<filename.bf>` with the path to the Brainfuck file you want to execute.

## Implementation

The interpreter is implemented according to the Wikipedia page on Brainfuck: https://en.wikipedia.org/wiki/Brainfuck.

The program reads in the contents of the Brainfuck file and converts each character to an instruction. It then executes the instructions using a `BrainfuckedState` struct, which keeps track of the current state of the interpreter.

The `BrainfuckedState` struct has three fields:

- `data`: A vector of bytes that represent the program's memory.
- `data_ptr`: An integer that points to the current location in `data`.
- `code_ptr`: An integer that points to the current location in the program's code.
- `instructions`: A vector of `Instruction` enums, each representing a Brainfuck instruction.

The `execute` method of the `BrainfuckedState` struct executes the instructions one by one until it reaches the end of the program.

## Supported Instructions

The interpreter supports the following Brainfuck instructions:

- `>`: Increment the data pointer.
- `<`: Decrement the data pointer.
- `+`: Increment the byte at the data pointer.
- `-`: Decrement the byte at the data pointer.
- `.`: Output the byte at the data pointer as a character.
- `,`: Input a character and store it in the byte at the data pointer.
- `[`: Jump past the matching `]` if the byte at the data pointer is zero.
- `]`: Jump back to the matching `[` if the byte at the data pointer is nonzero.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
