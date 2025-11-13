# IPL (Interpreted Programming Language)

A simple interpreted programming language implemented in Rust.

## Table of Contents
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Syntax](#syntax)
  - [Contributing](#contributing)
  - [License](#license)

---

## Features

- **Variables**: Dynamic variable assignment 
- **Control Flow**: 
  - `if`/`elif`/`else` conditionals
  - `while` loops with `break`/`continue`
  - `for` loops with iteration over lists
  - `break` and `continue` statements
- **Functions**:
  - Function definitions with parameters
  - Return values
  - Built-in functions for I/O and math
- **Data Types**:
  - Numbers (floating point)
  - Strings
  - Lists
  - Booleans (`True`/`False`)
  - `None`
- **Classes & Objects**:
  - Class definitions with inheritance
  - Constructor methods
  - Instance methods
  - Class variables and instance variables
  - Method overriding
- **File Imports**: Import code from other `.ipl` files
- **Operators**:
  - Arithmetic: `+`, `-`, `*`, `/` 
  - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Logical: `and`, `or`
  - Member access: `.`

---

## Installation

1. Ensure you have Rust installed
2. Clone the repository
3. Build the project:
```sh
cargo build --release
```
or

Download the [latest release](https://github.com/I-had-a-bad-idea/IPL/releases)

---

## Usage

Run an IPL file:
```sh
cargo run path/to/file.ipl
```
or´(if you have the executable)

```sh
IPL.exe path/to/file.ipl
```

---

## Syntax

An explanation for syntax of IPL can be found [here](SYNTAX.md).

Syntax highlighting can be found [here](https://github.com/I-had-a-bad-idea/IPL-Highlighting)

---

## Contributing

Contributions are welcome! Please refer to [Contributing](CONTRIBUTING.md)

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
