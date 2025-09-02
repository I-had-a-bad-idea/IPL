class Evaluator:

    def ev_file(self, file_path):
        file = open(file_path)
        lines = file.read().split("\n")
        
        self.variables = {}

        for line in lines:
            if line.strip() == "":
                continue

            variable_name, expr = line.split("=", maxsplit=2)
            variable_name = variable_name.strip()
            self.variables[variable_name] = self.ev_expr(expr)
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
            elif token == "+":
                lhs = float(stack.pop())
                rhs = tokens[i + 1]
                if rhs.isdigit():
                    rhs = float(tokens[i + 1])
                else:
                    rhs = float(self.variables[tokens[i + 1]])
                stack.append(lhs + rhs)
                i += 1

            i += 1

        return stack[0]