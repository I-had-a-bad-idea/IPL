![MIT License](https://img.shields.io/badge/license-MIT-green)
![Python](https://img.shields.io/badge/Rust-1.89.0%2B-red)
![Commit Activity](https://img.shields.io/github/commit-activity/m/I-had-a-bad-idea/IPL)
![Last Commit](https://img.shields.io/github/last-commit/I-had-a-bad-idea/IPL)
![Open Issues](https://img.shields.io/github/issues/I-had-a-bad-idea/IPL)
![Closed Issues](https://img.shields.io/github/issues-closed/I-had-a-bad-idea/IPL)
![Repo Size](https://img.shields.io/github/repo-size/I-had-a-bad-idea/IPL)
![Contributors](https://img.shields.io/github/contributors/I-had-a-bad-idea/IPL)


# IPL (Interpreted Programming Language)

A simple interpreted programming language implemented in Rust.

## Table of Contents
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Syntax](#syntax)
  - [Libraries](#libraries)
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

## Libraries

IPL has it´s own library installer: [ILI](https://github.com/I-had-a-bad-idea/ILI) (IPL-Library-Installer).

## Contributing

Contributions are welcome! Please refer to [Contributing](CONTRIBUTING.md)

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
