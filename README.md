# IPL (Interpreted Programming Language)

A simple interpreted programming language implemented in Rust that supports object-oriented programming concepts, control flow, and built-in functions.

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
- `in(prompt)`: Get user input with an prompt
- `random(start, end)`: Generate random integer between start and end (inclusive)
- `round(number)`: Round a number to nearest integer
- `pow(base, exp)`: Calculate base raised to exp power
- `min(list)`: Get minimum value from a list of numbers
- `max(list)`: Get maximum value from a list of numbers
- `len(collection)`: Get length of string or list
- `value(number)`: Convert to number type

## Example Programs

More example programms can be found under `examples/`.

### Hello World
```ipl
out("Hello, World!")
```

### Class Inheritance
```ipl
class Animal
    self.name = "Unknown"
    
    def make_sound()
        out("Some sound")

class Dog : Animal
    def Dog(name)
        self.name = name
    
    def make_sound()
        out("Woof!")

dog = Dog("Rex")
dog.make_sound()  # Prints: Woof!
out(dog.name)     # Prints: Rex
```

### List Operations
```ipl
numbers = [1, 2, 3, 4, 5]
sum = 0

for n in numbers
    sum = sum + n

out("Sum is:")
out(sum)

min_val = min(numbers)
max_val = max(numbers)
```

## Contributing

Contributions are welcome! Please refer to [Contributing](CONTRIBUTING.md)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.