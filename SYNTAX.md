
# IPL — Language Syntax Reference

This document is a concise, example-driven reference for the IPL syntax used by the interpreter in this repository. It summarizes the lexical rules, statement and expression forms, function and class syntax, modules/imports, built-ins, and commonly-seen idioms from the `examples/` directory.

If anything below looks different from what you expect, tell me.

## Quick overview

- IPL is indentation-significant: block structure is determined by consistent indentation (spaces are recommended).
- Statements are line-oriented; block headers (like `def`, `if`, `class`, etc.) are followed by an indented block on subsequent line(s).
- The language uses familiar Python-like operators and function call syntax: `name(args)`.

## 1. Lexical elements

- Comments: `#` starts a comment that runs to the end of the line.
- Identifiers: start with a letter or `_`, followed by letters, digits or `_` (e.g. `my_var`, `_internal`).
- Line termination: statements end at the newline. There is no semicolon terminator.
- Indentation: use consistent spaces to indicate nested blocks. All lines in the same block must share the same indentation level.

## 2. Literals

- Numbers: integer-like numerals (like `0`, `5`).
- Strings: double-quoted strings: `"Hello"`.
- Lists: list literals are supported (examples use `list = [1, 2, 3]`).
- Booleans / none: `true`/`false`/`none` tokens or capitalized

Example literals:

```
num = 10
name = "Tom"
list = [1, 2, 3]
```

## 3. Expressions and operators

- Arithmetic: `+`, `-`, `*`, `/` (e.g. `a + b`).
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`.
- Logical operators: `and`, `or`. 

Operator examples:

```
sum = a + b * 2
if a == b
    out("equal")
```

## 4. Assignment

Assign with `=`:

```
x = 5
name = "Bob"
obj.field = 3
```

Instance fields are assigned using `self` inside class blocks (see classes section).

## 5. Functions

Function definitions use `def name(params)` with the body indented beneath the header. There is no trailing colon. Use `return` to return a value; if omitted, functions return nothing.

```
def add(a, b)
    return a + b

def greet()
    out("Hi")
```

Function calls are `add(1, 2)`.

Notes:
- Parameter lists use parentheses. Empty parameter lists are `()`.
- Nested `if` / other blocks are expressed using deeper indentation.

## 6. Control flow

If/elif/else:

```
if x > 10
    out("more than 10")
elif x == 10
    out("ten")
else
    out("les than 10")
```

Loops:

- `while` loops:

```
i = 0
while i <= 5
    out(i)
    i = i + 1
```

- `for` loops over iterables:

```
list = [1, 2, 3, 4, 5]
for number in list
    out(number)
```

Control keywords: `continue`  `break` work just like in other languages.

## 7. Lists and iteration

Lists are written with square brackets and comma-separated elements: `[1, 2, 3]`. `for <var> in <iterable>` iterates over lists or other iterable values.

Indexing and slicing syntax are currently not implemented.

## 8. Classes and objects

Class definition syntax:

```
class Class : Base
    self.field = 0

    def method(arg)
        ...

    def Class(arg1, arg2)
        # constructor: method with same name as class
        self.field = arg1

obj = Class()
obj.method()
```

- `class Name` or `class Name : Base` — colon separates the base class.
- Inside a class body, `self` is used to define instance fields: `self.name = "John"`.
- Methods are regular function blocks inside the class.
- Constructors are implemented as a method named the same as the class (e.g. `def Person(n, a)` inside `class Person`), and are invoked via `p = Person("Alex", 30)`.
- Create an instance with `t = Test()` and call a method with `t.greet()`.

9. Static functions

Static functions are defined just like regular functions inside a class, but they do not access instance fields.
They are called using the class name, e.g. Class.function().
No special keyword (like static) is needed — any method called on a class is treated as static (this may produce errors, if you use self in the function):

Example:

```
class Class
    self.field = 0

    def method(arg)
        ...


Class.method()
```

## 10. Modules / Import

Import uses a simple filename form in the examples:

```
import utils.ipl
import class_test_base_class.ipl
```

The imported filename is looked up relative to the current file.

## 11. Built-ins and common functions

Built-ins include:

- `out(value)`: Print a value to stdout
- `in(prompt)`: Get user input with an prompt
- `random(start, end)`: Generate random integer between start and end (inclusive)
- `round(number)`: Round a number to nearest integer
- `pow(base, exp)`: Calculate base raised to exp power
- `min(list)`: Get minimum value from a list of numbers
- `max(list)`: Get maximum value from a list of numbers
- `len(collection)`: Get length of string or list
- `value(number)`: Convert to number type
  
These are called like normal functions (e.g. `out("Hello World")`)


## 12. Example snippets

Hello world:

```
out("Hello World")
```

Guess-the-number:

```
out("Welcome to Guess-the-number")
random_number = random(0, 10)
input = in("Please enter a number between 0 and 10: ")
if input == random_number
    out("You guessed the number.")
else
    out("You didnt guess the number.")
    out(random_number)
```

Class + constructor example:

```
class Person
    self.name = "John"
    self.age = 20

    def Person(n, a)
        self.name = n
        self.age = a

p = Person("Alex", 30)
```

For loop example:

```
list = [1, 2, 3, 4]
for number in list
    if number == 2
        continue
    out(number + 2)
```


---

If any questions remain please open an issue, so I can add it to this file.
