import sys
import os

from evaluator import Evaluator


file_path = sys.argv[1]

if file_path.split(".")[1] != "txt":  #TODO use actual file extension/name
    print("Not a file in this language")
    sys.exit()

if not os.path.isfile(file_path):
    print("File doesnt exist")
    sys.exit()

evaluator = Evaluator()

evaluator.ev_file(file_path)
