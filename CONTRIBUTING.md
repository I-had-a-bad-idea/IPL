# Contributing to IPL

Thank you for your interest in contributing to **IPL**, a simple interpreted programming language written in Rust! Whether you're fixing bugs, improving documentation, or building new features, your help is appreciated.

## Table of Contents

- [Contributing to IPL](#contributing-to-ipl)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
  - [Project Structure](#project-structure)
  - [Contribution Guidelines](#contribution-guidelines)
    - [Do:](#do)
    - [Don't:](#dont)
  - [Areas to Contribute](#areas-to-contribute)
    - [High Priority](#high-priority)
    - [Future Features](#future-features)
    - [Low Priority](#low-priority)
  - [Code Style \& Best Practices](#code-style--best-practices)
  - [Pull Request Process](#pull-request-process)
  - [License](#license)

---

## Getting Started

1. **Fork the repository** to your own GitHub account.

2. **Clone** your fork locally:

   ```sh
   git clone https://github.com/your-username/IPL.git
   cd ipl
   ```

3. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

4. **Build the project**:

   ```sh
   cargo build --release
   ```

5. **Run a sample IPL program**:

   ```sh
   cargo run path/to/your_file.ipl
   ```

---

## Project Structure

```
src/
├── main.rs              # Entry point
├── evaluator.rs         # Core interpreter logic
├── tokenizer.rs         # Preprocessing and tokenization
├── built_in_functions.rs # Built-in function implementations
├── debug.rs             # Error handling
└── state.rs             # Global program state (line, line_content)

test/
├── common/
|      ├── mod.rs         # Contains a funtion to run IPL files
├── ipl_files/            # IPL files for the tests
├── *.rs                  # Tests
```

---

## Contribution Guidelines

### Do:

* Follow Rust coding conventions and formatting (`cargo fmt`)
* Write clear, descriptive commit messages
* Add comments and documentation when appropriate
* Write tests for new features or bug fixes
* Keep pull requests focused and minimal

### Don't:

* Include unrelated changes in a PR
* Leave commented-out or dead code, except debug prints, that could be used later
* Push directly to `main` (open a PR instead)

---

## Areas to Contribute

### High Priority
* Make indexing
* Refactor code for clarity and maintainability
* Add more built-in functions
* Write more tests
* Add documentation

### Future Features

* Support static/class methods
* Access modifiers (public/private/protected)
* Namespaces or module system
* Standard library development
* Syntax highlighting support for:

  * VSCode (already in development)
  * Sublime Text
  * Vim

### Low Priority

These aren't essential right now, but contributions here are still welcome:
* Add more built-ins to the SYNTAX.md
* Improve performance of the interpreter (e.g. optimize evaluation logic)
* Add more examples to the examples/ directory
* Write benchmark tests for interpreter performance
* Improve error messages
* Add .ipl file syntax checking (ipl check file.ipl)
* Add debug flags (e.g. --trace, --ast)

---

## Code Style & Best Practices

* Format with `cargo fmt`
* Lint with `cargo clippy`
* Use meaningful function and variable names
* Prefer readability over cleverness

---

## Pull Request Process

1. Create a new branch:

   ```sh
   git checkout -b feature/YourFeatureName
   ```

2. Make your changes

3. Test thoroughly

4. Commit and push:

   ```sh
   git commit -m "Add: Your feature summary"
   git push origin feature/YourFeatureName
   ```

5. Open a Pull Request to `main` with a clear title and description

6. I will review your PR

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

**Thanks again for helping improve IPL!**

If you have any questions or need clarification, feel free to open an issue or start a discussion.

---
