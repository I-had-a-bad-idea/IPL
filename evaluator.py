
def get_indentation(line):
    return len(line) - len(line.lstrip())

class Evaluator:


    def ev_file(self, file_path):
        file = open(file_path)
        lines = [x for x in file.read().split("\n") if x.rstrip() != ""]  # Get all lines, which are not empty
        
        lines.append("End of file")

        self.variables = {}
        programm_counter = 0 # The line counter

        indentation_stack = [("normal", 0)]

        while programm_counter < len(lines):
            line = lines[programm_counter]
            line = line.split("#")[0]   # Ignore comments

            indentation = get_indentation(line)

            if indentation <= indentation_stack[-1][1]:
                if indentation_stack[-1][0] == "while":
                    while lines[programm_counter].split(maxsplit=1)[0] != "while":
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
                        while get_indentation(lines[programm_counter]) > indentation_stack[-1][1]:
                            programm_counter += 1 # Go forward until end of loop
                
                case "if":
                    if self.ev_expr(line.split(maxsplit=1)[1]) == True:
                        programm_counter += 1 # Enter if statement
                        indentation_stack.append(("if", indentation))
                    else:
                        programm_counter += 1 # Enter statement body
                        while get_indentation(lines[programm_counter]) > indentation_stack[-1][1]:
                            programm_counter += 1 # Skip if statement
                        if lines[programm_counter].split(maxsplit=1)[0] == "else" and get_indentation(lines[programm_counter]) == indentation:
                            programm_counter += 1
                            indentation_stack.append(("else", indentation))
                case "else":
                    programm_counter += 1
                    while get_indentation(lines[programm_counter]) > indentation_stack[-1][1]:
                        programm_counter += 1
                case _:
                    if line == "End of file":
                        break

                    variable_name, expr = line.split("=", maxsplit=2)
                    variable_name = variable_name.strip()
                    self.variables[variable_name] = self.ev_expr(expr)
                    programm_counter += 1
        print(f"variables {self.variables}")


    def ev_expr(self, line):
        tokens = line.split()
        stack = []
        i = 0
        while i < len(tokens):
            print(f"at index {i} the stack is {stack} with tokens {tokens}")
            token = tokens[i]
            if token.isdigit():
                stack.append(token)
            elif token in self.variables:
                stack.append(self.variables[token])
            else:
                lhs = float(stack.pop())
                rhs = tokens[i + 1]
                if rhs.isdigit():
                    rhs = float(tokens[i + 1])
                else:
                    rhs = float(self.variables[tokens[i + 1]])
                
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



                i += 1

            i += 1

        return stack[0]