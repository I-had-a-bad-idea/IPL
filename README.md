# IPL (Interpreted Programming Language)

A simple interpreted programming language implemented in Rust. IPL supports basic programming constructs like variables, functions, control flow, and built-in functions.

## Features

- **Variables**: Dynamic variable assignment
- **Control Flow**: 
  - `if`/`elif`/`else` conditionals
  - `while` loops 
  - `for` loops
  - `break` and `continue` statements
- **Functions**:
  - Function definitions with parameters
  - Return values
  - Built-in functions
- **Data Types**:
  - Numbers (floating point)
  - Strings
  - Lists
  - Booleans (`True`/`False`)
  - `None`
- **Classes & Objects**:
  - Class definitions
  - Object instantiation
  - Fields/attributes
  - Default values
- **File Imports**: Import code from other `.ipl` files
- **Operators**:
  - Arithmetic: `+`, `-`, `*`, `/`
  - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Logical: `and`, `or`

## Installation

1. Ensure you have Rust installed
2. Clone the repository
3. Build the project:
```sh
cargo build --release
```

## Usage

Run an IPL file:
```sh
cargo run path/to/file.ipl
```

## Built-in Functions

- `out(value)`: Print a value to stdout
- `in(prompt)`: Get user input with a prompt
- `random(start, end)`: Generate random number between start and end
- `round(number)`: Round a number to nearest integer
- `pow(base, exp)`: Calculate base raised to exp power
- `min()`: Get minimum value (placeholder)
- `max()`: Get maximum value (placeholder)
- `value(number)`: Convert to number

## Example Programs

### Hello World
```ipl
out("Hello, World!")
```

### Guess the Number Game
```ipl
random_number = random(0,10)
input = in("Guess a number between 0-10: ")

if input == random_number
    out("You guessed it!")
else
    out("Wrong guess!")
out("The number was: ")
out(random_number)
```

### Function Definition and Loops
```ipl
def add(a,b)
    return a + b

list = [1, 2, 3, 4]
for number in list
    out(add(number, 2))
```

## Classes

Classes can be defined with fields and default values:

```ipl
class Person
    name = "John"
    age = 30

# Create an instance
p1 = Person()
out(p1.name)  # Prints: John

# Modify fields
p1.name = "Alice"
p1.age = 25

# Create another instance
p2 = Person()
out(p2.name)  # Prints: John (default value)
out(p2.age)   # Prints: 30 (default value)
```

### Class Features
- Define classes with default field values
- Create multiple instances
- Access and modify instance fields using dot notation
- Each instance maintains its own state
- Fields can be numbers, strings, lists, or other values

## Project Structure

- `src`
  - `src/main.rs`: Entry point
  - `src/evaluator.rs`: Core interpreter logic
  - `src/tokenizer.rs`: Lexical analysis
  - `src/built_in_functions.rs`: Built-in function implementations
  - `src/debug.rs`: Error handling

## License

MIT License - See `LICENSE` for details

## Contributing

Contributions welcome! Current TODOs:
- Add more built-in functions
- Make syntax highlighting
- 
- Access control (public/protected/private)
- Static and class methods
- 
- Make a debug print function 
- 
- WRITE COMMENTS
- Clean up and optimize code
