class Evaluator:

    def ev_file(self, file_path):
        file = open(file_path)
        lines = file.read().split("\n")
        
        for line in lines:
            if line.strip() != "":
                self.ev_line(line)

    def ev_line(self, line):
        tokens = line.split()
        stack = []
        i = 0
        while i < len(tokens) - 1:
            if tokens[i].isdigit():
                stack.append(tokens[i])
            elif tokens[i] == "+":
                lhs = stack.pop()
                rhs = tokens.pop(i + 1)
                stack.append(float(lhs) + float(rhs))

            i += 1

        print(stack)