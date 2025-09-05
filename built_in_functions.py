import random
from error import EvaluationError

built_in_functions = {
    "out": ["output"],
    "value": ["number"],
    "in": ["message"],
    "random": ["start", "end"],
    "min": [],
    "max": [],
    "round": ["number"],
    "pow": ["base", "exp"],

}

def call_built_in_function(function_name, arguments = []):
    try:
        match function_name:
            case "out":
                print(arguments[0])
            case "value":
                return abs(arguments[0])
            case "in":
                return input(arguments[0])
            case "random":
                return random.randrange(arguments[0], arguments[1])
            case "min":
                return min(arguments)
            case "max":
                return max(arguments)
            case "round":
                return round(arguments[0])
            case "pow":
                return pow(arguments[0], arguments[1])
    except Exception as e:
        raise EvaluationError(e)