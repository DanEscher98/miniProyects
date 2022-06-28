import collections
import inspect
import types

import six

PY3, PY2 = six.PY3, not six.PY3


class Interpreter:
    def __init__(self):
        self.stack = []

    def LOAD_VALUE(self, number):
        self.stack.append(number)

    def PRINT_ANSWER(self):
        answer = self.stack.pop()
        print(answer)

    def ADD_TWO_VALUES(self):
        fst_num = self.stack.pop()
        snd_num = self.stack.pop()
        total = fst_num + snd_num
        self.stack.append(total)

    def run_code(self, program):
        instructions = program["instructions"]
        data = program["data"]

        for each_step in instructions:
            instruction, argument = each_step
            match instruction:
                case "LOAD_VALUE":
                    number = data[argument]
                    self.LOAD_VALUE(number)
                case "ADD_TWO_VALUES":
                    self.ADD_TWO_VALUES()
                case "PRINT_ANSWER":
                    self.PRINT_ANSWER()


class VirtualMachineError(Exception):
    pass


class VirtualMachine:
    """Only one instance will be created each time the
    program is run. The entry point is the method `run_code`"""

    def __init__(self):
        self.frames = []  # The call stack of frames
        self.frame = None  # The current frame
        self.return_value = None
        self.last_exception = None

    def run_code(self, code, global_names=None, local_names=None):
        """An entry point to execute code using the VM"""
        frame = self.make_frame(
            code, global_names=global_names, local_names=local_names
        )
        self.run_frame(frame)

    def make_frame(self, code, global_names, local_names):
        pass

    def run_frame(self, frame):
        pass


class Frame:
    """The frame is a collection of attributes with no
    methods. One of its attributes include the code object
    generated by the compiler"""

    # pylint: disable=too-many-instance-attributes
    # pylint: disable=too-few-public-methods
    # Eight is reasonable in this case.

    def __init__(self, code_obj, global_names, local_names, prev_frame):
        self.code_obj = code_obj
        self.global_names = global_names
        self.local_names = local_names
        self.prev_frame = prev_frame
        self.stack = []
        self.builtin_names = (
            prev_frame.builtin_names
            if prev_frame
            else (
                self.builtin_names.__dict__
                if hasattr(self.builtin_names, "__dict__")
                else local_names["__builtins__"]
            )
        )
        self.last_instruction = 0
        self.block_stack = []


class Function:
    """Create a realistic function object, defining the
    things the interpreter expects"""

    # pylint: disable=too-many-arguments

    __slots__ = [
        "func_code",
        "func_name",
        "func_defaults",
        "func_globals",
        "func_locals",
        "func_dict",
        "func_closure",
        "__name__",
        "__dict__",
        "__doc__",
        "_vm",
        "_func",
    ]

    def __init__(self, name, code, globs, defaults, closure, vm):
        self._vm = vm
        self.func_code = code
        self.func_name = self.__name__ = name or code.co_name
        self.func_defaults = tuple(defaults)
        self.func_globals = globs
        self.func_locals = self._vm.frame.f_locals
        self.__dict__ = {}
        self.func_closures = closure
        self.__doc__ = code.co_consts[0] if code.co_consts else None

        # Sometimes, we need a real Python function. This is
        # for that
        kw = {"argdefs": self.func_defaults}
        if closure:
            kw["closure"] = tuple(make_cell(0) for _ in closure)
        self._func = types.FunctionType(code, globs, **kw)

    def __call__(self, *args, **kwargs):
        """When calling a Function, make a new frame and run it"""
        # callargs = inspect.getcallargs(self._func, *args, **kwargs)
        callargs = dict(inspect.signature(self._func).bind(*args, **kwargs).arguments)
        # Use callargs to provide a mapping of arguments:
        # values to pass into the new frame
        frame = self._vm.make_frame(self.func_code, callargs, self.func_globals, {})
        return self._vm.run_frame(frame)


def make_cell(value):
    """Create a real Python closure and grab a cell"""
    # pylint: disable=unnecessary-direct-lambda-call
    # pylint: disable=no-member
    # pylint: disable=unsubscriptable-object
    func = (lambda x: lambda: x)(value)
    if not PY3:
        return func.func_closure[0]
    return func.__closure__[0]