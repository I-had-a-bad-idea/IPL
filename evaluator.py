from error import EvaluationError

def get_indentation(line):
    return len(line) - len(line.lstrip())

class Evaluator:


    def ev_file(self, file_path):
        file = open(file_path)
        self.lines = [x for x in file.read().split("\n") if x.rstrip() != ""]  # Get all lines, which are not empty
        
        self.lines.append("End of file")

        self.variables = {}
        self.functions = {}
        programm_counter = 0 # The line counter

        indentation_stack = [("normal", 0)]

        while programm_counter < len(self.lines):
            line = self.lines[programm_counter]
            line = line.split("#")[0]   # Ignore comments

            indentation = get_indentation(line)

            if indentation <= indentation_stack[-1][1]:
                if indentation_stack[-1][0] == "while":
                    while self.lines[programm_counter].split(maxsplit=1)[0] != "while":
                        programm_counter -= 1
                    indentation_stack.pop()
                    continue
                elif indentation_stack[-1][0] == "if":
                    indentation_stack.pop()
                    
                elif indentation_stack[-1][0] == "else":
                    indentation_stack.pop()


            match line.split(maxsplit=1)[0]: # Get the first word
                case "while":
                    if self.ev_expr(line.split(maxsplit=1)[1]) == True:
                        programm_counter += 1 # Enter loop body
                        indentation_stack.append(("while", indentation))
                    else:
                        programm_counter += 1  # Enter loop body
                        while get_indentation(self.lines[programm_counter]) > indentation_stack[-1][1]:
                            programm_counter += 1 # Go forward until end of loop
                
                case "if":
                    if self.ev_expr(line.split(maxsplit=1)[1]) == True:
                        programm_counter += 1 # Enter if statement
                        indentation_stack.append(("if", indentation))
                    else:
                        programm_counter += 1 # Enter statement body
                        while get_indentation(self.lines[programm_counter]) > indentation_stack[-1][1]:
                            programm_counter += 1 # Skip if statement
                        if self.lines[programm_counter].split(maxsplit=1)[0] == "else" and get_indentation(self.lines[programm_counter]) == indentation:
                            programm_counter += 1
                            indentation_stack.append(("else", indentation))
                case "else":
                    programm_counter += 1
                    while get_indentation(self.lines[programm_counter]) > indentation_stack[-1][1]:
                        programm_counter += 1

                case "def":
                    function_decleration = line.split(maxsplit=1)[1]
                    function_name = function_decleration.split("(", maxsplit=1)[0]
                    function_arguments = function_decleration.split("(", maxsplit=1)[1].strip(")").split(",")
                    programm_counter += 1
                    start_line = programm_counter
                    while get_indentation(self.lines[programm_counter]) > indentation:
                        programm_counter += 1
                    function_lines = list(range(start_line, programm_counter))
                    self.functions[function_name] = {"arguments": function_arguments, "function_body": function_lines}
                    print(f"functions:{self.functions}")
                case _:
                    if line == "End of file":
                        break
                    
                    self.ev_line(programm_counter)
                    programm_counter += 1
        print(f"variables {self.variables}")


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
            if token.strip(".").isdigit() or token in self.variables or token.split("(")[0] in self.functions:
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

    def ev_line(self, line): 
        variable_name, expr = self.lines[line].split("=", maxsplit=2)
        variable_name = variable_name.strip()
        self.variables[variable_name] = self.ev_expr(expr)

    def ev_func(self, function_name, arguments = []):
        function_arguments = self.functions[function_name]["arguments"]
        function_lines = self.functions[function_name]["function_body"]

        if len(arguments) != len(function_arguments):
            raise EvaluationError("Wrong amount of arguments")
        
        global_variables = self.variables

        for name, value in zip(function_arguments, arguments):
            self.variables[name] = value

        result = None

        for function_line in function_lines:
            if self.lines[function_line].split(maxsplit=1)[0] == "return":
                expr = self.lines[function_line].split("return", maxsplit=1)[1]
                result = self.ev_expr(expr)
                break
            else:
                self.ev_line(function_line)

        self.variables = global_variables

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
            
            elif token.split("(")[0] in self.functions:
                if "(" in token and token.endswith(")"):
                    function_name = token.split("(", 1)[0]
                    argument_str = token.split("(", 1)[1].rstrip(")")
                    argument_values = [self.ev_expr(a.strip()) for a in argument_str.split(",") if a]
                    stack.append(self.ev_func(function_name, argument_values))
                else:
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