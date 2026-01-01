
# IPL — Language Syntax Reference

This document is a concise reference for the IPL syntax used by the interpreter in this repository. It summarizes the lexical rules, statement and expression forms, function and class syntax, modules/imports, built-ins, and more.

If anything below looks different from what you expect, tell me.

## Table of Contents

- [IPL — Language Syntax Reference](#ipl--language-syntax-reference)
  - [Table of Contents](#table-of-contents)
  - [0. Quick overview](#0-quick-overview)
  - [1. Lexical elements](#1-lexical-elements)
  - [2. Literals](#2-literals)
  - [3. Expressions and operators](#3-expressions-and-operators)
  - [4. Assignment](#4-assignment)
  - [5. Functions](#5-functions)
  - [6. Control flow](#6-control-flow)
    - [6.1 If/elif/else:](#61-ifelifelse)
    - [6.2 Loops:](#62-loops)
      - [6.2.1 while loops:](#621-while-loops)
      - [6.2.2 for loops](#622-for-loops)
      - [6.2.3 Continue](#623-continue)
  - [7. Lists, iteration and indexing](#7-lists-iteration-and-indexing)
    - [7.1 Lists](#71-lists)
    - [7.2 Iteration](#72-iteration)
    - [7.3 Indexing and slicing](#73-indexing-and-slicing)
  - [8. Classes and objects](#8-classes-and-objects)
    - [8.1 Definition and usage](#81-definition-and-usage)
    - [8.2 Instances](#82-instances)
    - [8.3 Static functions](#83-static-functions)
  - [9. Modules / Import](#9-modules--import)
  - [10. Libraries](#10-libraries)
    - [10.1 ILI](#101-ili)
    - [10.2 Using libraries](#102-using-libraries)
  - [11. Built-in functions](#11-built-in-functions)
  - [12. Example snippets](#12-example-snippets)
    - [12.1 Hello world:](#121-hello-world)
    - [12.2 Guess-the-number:](#122-guess-the-number)
    - [12.3 Class + constructor example:](#123-class--constructor-example)
  - [Conclusion](#conclusion)

## 0. Quick overview

- IPL is indentation-significant: block structure is determined by consistent indentation.
- Statements are line-oriented; block headers (like `def`, `if`, `class`, etc.) are followed by an indented block on subsequent line(s).
- The language is Python-like, but with simplified syntax.

## 1. Lexical elements

- Comments: `#` starts a comment that runs to the end of the line.
- Identifiers: start with a letter or `_`, followed by letters, digits or `_` (e.g. `my_var`, `_internal`).
- Line termination: statements end at the newline (end of line). There is no semicolon terminator.
- Indentation: use consistent indentation to indicate nested blocks. All lines in the same block must share the same indentation level.

## 2. Literals

- Numbers: integer-or-float-like numerals (like `0`, `5`, `3.141`).
- Strings: double-quoted strings: `"Hello"` or single-quoted strings: `'Hello'` .
- Lists: list literals are supported (`list = [1, 2, 3]`).
- Booleans / none: `true`/`false`/`none` tokens or capitalized

Example literals:

```
num = 10
name = "Tom"
list = [1, 2, 3]
```

## 3. Expressions and operators

- Arithmetic: `+`, `-`, `*`, `/` 
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

Instance fields are assigned using `self` inside class blocks (see [classes section](#8-classes-and-objects)).

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

### 6.1 If/elif/else:

If statements consist of two parts `if` and a condition following it.           
If the following condition is `true` the following indented block is executed.               
If it is `false` **IPL** will check if there is an `elif`.               
If it finds one it treats it like an `if`.              
If all `if`s and `elif`s are `false` the indented block beneath `else` will be executed.               

```
if x == 10
    out("x is ten")
elif x > 10
    out("x is more than 10")
else
    out("x is less than 10")
```

### 6.2 Loops:

#### 6.2.1 while loops:

`While` loops are executed as long as the condition following the `while` is `true`.
 
```
i = 0
while i <= 5
    out(i)
    i = i + 1

# Output: 
#   0
#   1
#   2
#   3
#   4
#   5
```

#### 6.2.2 for loops

`for <var> in <iterable>` iterates over iterables.   
Each iteration assigns the next element to `<var>`.  

```
list = [1, 2, 3, 4, 5]
for number in list
    out(number)

# Output:
#   1
#   2
#   3
#   4
#   5
```

Control keywords: `continue`  `break` work just like in other languages.

#### 6.2.3 Continue

Continue ends the current iteration and skips to the next one.

Example:

```
list = [1, 2, 3, 4, 5]
for number in list
    if number == 2 or number == 4
        continue
    out(number)

# Ouput:
#   1
#   3
#   5

```


## 7. Lists, iteration and indexing

### 7.1 Lists

Lists are written with square brackets and comma-separated elements: `[1, 2, 3]`.

### 7.2 Iteration

See [for loops](#622-for-loops)

### 7.3 Indexing and slicing
Indexing looks like this:

```
list = [1, 2, 3, 4, 5]
value = list[0]        # value is 1
sublist = list[1:4]    # sublist is [2, 3, 4, 5]
```

There are two ways to index/slice:
- Single index: `list[index]` gets the element at `index` (0-based).
- Slice: `list[start:end]` gets a sublist from `start` (inclusive) to `end` (inclusive).


## 8. Classes and objects

### 8.1 Definition and usage

Class definition syntax:

```
class Class : Base
    self.field = 0

    def method(arg)
        ...

    def Class(arg1, arg2)
        # constructor: method with same name as class
        self.field = arg1

```

- `class Name` or `class Name : Base` — colon separates the base class.
- Inside a class body, `self` is used to define instance fields: `self.name = "John"`.
- Methods are regular function blocks inside the class.
- Constructors are implemented as a method named the same as the class (e.g. `def Person(n, a)` inside `class Person`), and are invoked via `p = Person("Alex", 30)`.

### 8.2 Instances

Class in this example is the Class from [here](#81-definition-and-usage)

Create an instance with `obj = Class()`.                
This will create a new `Class` and assign it to `obj`.          
You can now assign and get the field on the instance:

```
value = obj.field    # 0
obj.field = 25
```

To call methods on instances just do `obj.method(arg)`.             
It functions the same as a regular function, only that you call it on the instance.             
If the function modifies any fields, those will be modified on the instance.


### 8.3 Static functions

- Static functions are defined just like regular functions inside a class, but they do not access instance fields.
- They are called using the class name, e.g. `Class.function()`.
- No special keyword (like static) is needed — any method called on a class is treated as static when called on the class (this may produce errors, if you use self in the function, there are no checks for this currently).:

Example:

```
class Class
    self.field = 0

    def method(arg)
        ...


Class.method()
```

## 9. Modules / Import

Modules can be imported using the `import` keyword followed by the filepath (without `.ipl` extension).

Example:

```
import utils

number = add(5, 10) # assuming utils.ipl defines add function

```

The imported filename is looked up relative to the current file.
Once imported, the module’s variables and functions can be used directly in your IPL code.
There is no need for a namespace prefix.

## 10. Libraries

### 10.1 ILI

IPL supports external libraries to extend functionality. Libraries must first be installed via [ILI (IPL Library Installer)](https://github.com/I-had-a-bad-idea/ILI).

### 10.2 Using libraries

To imprt libraries use the `use` keyword followed by the library name:

Example:

```
use examplelib

number = examplelib.add(5, 10)

```

Once imported, the library’s variables and functions can be used directly in your IPL code.
Use the library name as a prefix to access its functions and variables.

Notes:
- Library names are case-sensitive.
- Installed libraries are available globally to all IPL scripts on your system.
- You still need to write `use <library_name>` in the scripts.
- Using a library that has not been installed will result in a runtime error.

## 11. Built-in functions

Built-in functions include:

- `out(value)`: Print a value to stdout
- `in(prompt)`: Get user input with an prompt
- `random(start, end)`: Generate random number between start and end (inclusive)
- `round(number)`: Round a number to nearest integer
- `pow(base, exp)`: Calculate base raised to exp power
- `min(list)`: Get minimum value from a list of numbers
- `max(list)`: Get maximum value from a list of numbers
- `len(collection)`: Get length of a string or a list
- `value(number)`: Returns the absolute value of the number.
  
These are called like normal functions (e.g. `out("Hello World")`).


## 12. Example snippets

### 12.1 Hello world:

```
out("Hello World")
```

### 12.2 Guess-the-number:

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

### 12.3 Class + constructor example:

```
class Person
    self.name = "John"
    self.age = 20

    def Person(n, a)
        self.name = n
        self.age = a

p = Person("Alex", 30)
```

---

## Conclusion

If any questions remain please open an issue, so I can improve this document.
