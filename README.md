![Tests](https://github.com/IPL-Foundation/IPL/actions/workflows/test.yml/badge.svg)
![MIT License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/Rust-1.89.0%2B-red)
![Commit Activity](https://img.shields.io/github/commit-activity/m/IPL-Foundation/IPL)
![Last Commit](https://img.shields.io/github/last-commit/IPL-Foundation/IPL)
![Open Issues](https://img.shields.io/github/issues/IPL-Foundation/IPL)
![Closed Issues](https://img.shields.io/github/issues-closed/IPL-Foundation/IPL)
![Repo Size](https://img.shields.io/github/repo-size/IPL-Foundation/IPL)
![Contributors](https://img.shields.io/github/contributors/IPL-Foundation/IPL)

# IPL (Interpreted Programming Language)

IPL is a simple interpreted programming language with the goal to make syntax as simple as possible.

Since it is designed for learning programming you dont have to worry about complex syntax and can focus on understanding programming itself first.


## Table of Contents
- [IPL (Interpreted Programming Language)](#ipl-interpreted-programming-language)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Syntax](#syntax)
  - [Library Installer](#library-installer)
  - [Contributing](#contributing)
  - [License](#license)

---


## Features

Everything you would expect from a simple programming language:

- Dynamic variables, numbers, strings, lists, booleans, and `None`
- Control flow: `if`/`elif`/`else`, `while`, `for`, `break`, `continue`
- Functions with parameters, return values, and built-in I/O
- Classes with inheritance, constructors, methods, class/instance variables, and overriding
- File imports (`.ipl` files)
- External libraries via [ILI (IPL Library Installer)](https://github.com/IPL-Foundation/ILI)
- Operators: arithmetic, comparison, logical, and member access

---

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed
2. Clone the repository
3. Build the project:
```sh
cargo build --release
```
or

Download the [latest release](https://github.com/IPL-Foundation/IPL/releases)

---

## Usage

Run an IPL file:
```sh
cargo run path/to/file.ipl
```

or if you have the executable:
```sh
IPL.exe path/to/file.ipl
```

---

## Syntax

An explanation for syntax of IPL can be found [here](SYNTAX.md).

Syntax highlighting for VS-Code can be found [here](https://github.com/IPL-Foundation/IPL-Highlighting-VS-Code)

---

## Library Installer

IPL has it´s own library installer: [ILI (IPL-Library-Installer)](https://github.com/IPL-Foundation/ILI).

## Contributing

Contributions are welcome! Please refer to [Contributing](CONTRIBUTING.md)

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
