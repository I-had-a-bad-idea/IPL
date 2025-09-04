
built_in_functions = {
    "out": ["output"],
    "value": ["number"]
}

def out(output):
    print(output)

def value(number):
    return abs(number)

def call_built_in_function(function_name, arguments = []):
    match function_name:
        case "out":
            out(arguments[0])
        case "value":
            return value(arguments[0])
