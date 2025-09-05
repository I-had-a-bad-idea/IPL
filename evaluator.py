from error import EvaluationError
from built_in_functions import built_in_functions, call_built_in_function

def get_indentation(line):
    return len(line) - len(line.lstrip())

class Evaluator:


    def ev_file(self, file_path):
        self.file_path = file_path
        file = open(self.file_path)
        self.lines = [x for x in file.read().split("\n") if x.rstrip() != ""]  # Get all lines, which are not empty
        
        self.lines.append("End of file")

        self.indentation_stack = [("normal", 0)]

        self.variables = {}
        self.functions = {}
        self.evaluators = {}
        
        self.execute_lines(0, len(self.lines))
        print(f"variables {self.variables}")

    def execute_lines(self, start_line, end_line):
        programm_counter = start_line # The line counter

        print(f"execute_lines called with start = {start_line}, end = {end_line}")

        while programm_counter < end_line:
            line = self.lines[programm_counter]
            line = line.split("#")[0]   # Ignore comments

            indentation = get_indentation(line)

            if indentation <= self.indentation_stack[-1][1]:
                if self.indentation_stack[-1][0] == "while":
                    while self.lines[programm_counter].split(maxsplit=1)[0] != "while":
                        programm_counter -= 1
                    self.indentation_stack.pop()
                    continue
                elif self.indentation_stack[-1][0] == "if":
                    self.indentation_stack.pop()
                    
                elif self.indentation_stack[-1][0] == "else":
                    self.indentation_stack.pop()


            match line.split(maxsplit=1)[0]: # Get the first word
                case "import":
                    file = line.split(maxsplit=1)[1]
                    self.evaluators[file] = Evaluator()
                    self.evaluators[file].ev_file(file)
                    self.functions = {**self.functions, **self.evaluators[file].functions} # Merge function dicts
                    self.variables = {**self.variables, **self.evaluators[file].variables}

                    programm_counter += 1
                case "while":
                    if self.ev_expr(line.split(maxsplit=1)[1]) == True:
                        programm_counter += 1 # Enter loop body
                        self.indentation_stack.append(("while", indentation))
                    else:
                        programm_counter += 1  # Enter loop body
                        while get_indentation(self.lines[programm_counter]) > self.indentation_stack[-1][1]:
                            programm_counter += 1 # Go forward until end of loop
                
                case "if":
                    if self.ev_expr(line.split(maxsplit=1)[1]) == True:
                        programm_counter += 1 # Enter if statement
                        self.indentation_stack.append(("if", indentation))
                    else:
                        programm_counter += 1 # Enter statement body
                        while get_indentation(self.lines[programm_counter]) > self.indentation_stack[-1][1]:
                            programm_counter += 1 # Skip if statement
                        if self.lines[programm_counter].split(maxsplit=1)[0] == "else" and get_indentation(self.lines[programm_counter]) == indentation:
                            programm_counter += 1
                            self.indentation_stack.append(("else", indentation))
                case "else":
                    programm_counter += 1
                    while get_indentation(self.lines[programm_counter]) > self.indentation_stack[-1][1]:
                        programm_counter += 1
                
                case "return":
                    expr = line.split("return", maxsplit=1)[1]
                    print(f"Returning result of {expr}")
                    return self.ev_expr(expr)

                case "def":
                    function_decleration = line.split(maxsplit=1)[1]
                    function_name = function_decleration.split("(", maxsplit=1)[0]
                    function_arguments = function_decleration.split("(", maxsplit=1)[1].strip(")").split(",")
                    programm_counter += 1
                    start_line = programm_counter
                    while get_indentation(self.lines[programm_counter]) > indentation:
                        programm_counter += 1
                    function_lines = list(range(start_line, programm_counter))
                    self.functions[function_name] = {"file": self.file_path, "arguments": function_arguments, "function_body": function_lines}
                    print(f"functions:{self.functions}")
                case _:
                    if line == "End of file":
                        break
                    
                    if "=" in line:
                        variable_name, expr = line.split("=", maxsplit=1)
                        variable_name = variable_name.strip()
                        self.variables[variable_name] = self.ev_expr(expr)
                    else:
                        self.ev_expr(line)

                    programm_counter += 1

    def shunting_yard(self, tokens):
        prec = {
            "or": 1, "and": 2,
            "==": 3, "!=": 3, "<": 3, "<=": 3, ">": 3, ">=": 3,
            "+": 4, "-": 4,
            "*": 5, "/": 5
        }
        output = []
        stack = []

        for token in tokens:
            if token.strip(".").isdigit() or token in self.variables or token.split("(")[0] in self.functions or token.split("(")[0] in built_in_functions:
                output.append(token)
            elif token in prec:
                while stack and stack[-1] in prec and prec[stack[-1]] >= prec[token]:
                    output.append(stack.pop())
                stack.append(token)
            elif token == "(":
                stack.append(token)
            elif token == ")":
                while stack and stack[-1] != "(":
                    output.append(stack.pop())
                if not stack:
                    raise EvaluationError("Mismatched parentheses")
                stack.pop()  # remove "("
            else:
                raise EvaluationError(f"Unknown token {token}")

        while stack:
            if stack[-1] in ("(", ")"):
                raise EvaluationError("Mismatched parentheses")
            output.append(stack.pop())

        return output   

    def ev_func(self, function_name, arguments = []):
        file = self.functions[function_name]["file"]
        if file != self.file_path:
            return self.evaluators[file].ev_func(function_name, arguments)

        function_arguments = self.functions[function_name]["arguments"]
        function_lines = self.functions[function_name]["function_body"]

        print(f"Executing function {function_name} with lines: {function_lines}")
        print(f"Function lines content:")
        for i in function_lines:
            print(f"  {i}: '{self.lines[i]}'")

        if len(arguments) != len(function_arguments):
            raise EvaluationError("Wrong amount of arguments")
        
        global_variables = self.variables

        for name, value in zip(function_arguments, arguments):
            self.variables[name] = value

        self.indentation_stack.append(("function", get_indentation(self.lines[function_lines[0]])))

        result = self.execute_lines(function_lines[0], function_lines[-1] + 1)

        self.variables = global_variables

        self.indentation_stack.pop()

        return result

    def ev_expr(self, line):
        tokens = line.split()
        tokens = self.shunting_yard(tokens)
        stack = []

        for token in tokens:
            if token.isdigit():
                stack.append(token)
            elif token in self.variables:
                stack.append(self.variables[token])
            
            elif token.split("(")[0] in self.functions or token.split("(")[0] in built_in_functions:
                if "(" in token and token.endswith(")"):
                    function_name = token.split("(", 1)[0]
                    argument_str = token.split("(", 1)[1].rstrip(")")
                    argument_values = [self.ev_expr(a.strip()) for a in argument_str.split(",") if a]
                    if function_name in built_in_functions:
                        stack.append(call_built_in_function(function_name, argument_values))
                    elif function_name in self.functions:    
                        stack.append(self.ev_func(function_name, argument_values))
                else:
                    if token in built_in_functions:
                        stack.append(call_built_in_function(token))
                    elif token in self.functions:    
                        stack.append(self.ev_func(token))

            else:
                rhs = float(stack.pop())
                lhs = float(stack.pop())
                
                if token == "+":
                    stack.append(lhs + rhs)
                elif token == "-":
                    stack.append(lhs - rhs)
                elif token == "*":
                    stack.append(lhs * rhs)
                elif token == "/":
                    stack.append(lhs / rhs)
                
                elif token == "==":
                    stack.append(lhs == rhs)
                elif token == "!=":
                    stack.append(lhs != rhs)
                elif token == "<":
                    stack.append(lhs < rhs)
                elif token == "<=":
                    stack.append(lhs <= rhs)
                elif token == ">":
                    stack.append(lhs > rhs)
                elif token == ">=":
                    stack.append(lhs >= rhs)
                
                elif token == "and":
                    stack.append(lhs and rhs)
                elif token == "or":
                    stack.append(lhs or rhs)

        return stack[0]