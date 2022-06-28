from pyinterpreter.cli import get_args
from pyinterpreter.eval import Interpreter

prg1_simpleadd = {
    "instructions": [
        ("LOAD_VALUE", 0),
        ("LOAD_VALUE", 1),
        ("ADD_TWO_VALUES", None),
        ("LOAD_VALUE", 1),
        ("ADD_TWO_VALUES", None),
        ("PRINT_ANSWER", None),
    ],
    "data": [7, 5],
}

prg2_variables = {
    "instructions": [
        ("LOAD_VALUE", 0),
        ("STORE_NAME", 0),
        ("LOAD_VALUE", 1),
        ("STORE_NAME", 1),
        ("LOAD_NAME", 0),
        ("LOAD_NAME", 1),
        ("ADD_TWO_VALUES", None),
        ("PRINT_ANSWER", None),
    ],
    "data": [1, 2],
    "names": ["a", "b"],
}


def main():
    """Main program"""
    # _ = get_args()
    interpreter = Interpreter()
    interpreter.run_code(prg1_simpleadd)


if __name__ == "__main__":
    main()
