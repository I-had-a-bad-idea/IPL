class EvaluationError(Exception):
    def __init__(self, error_message, line_number = None, line_content = None):
        super().__init__(error_message)

        self.line_number = line_number
        self.line_content = line_content


    def __str__(self):
        base = super().__str__()
        if self.line_number:
            base += f"line {self.line_number}"
        if self.line_content:
            base += f"line content {self.line_content}"

        return base