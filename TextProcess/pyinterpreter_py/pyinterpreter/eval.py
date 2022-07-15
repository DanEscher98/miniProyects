import collections
import dis
import inspect
import sys
import types
from typing import Any, Tuple

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


Block = collections.namedtuple("Block", "type, handler, stack_height")


class VirtualMachineError(Exception):
    pass

Block = collections.namedtuple("Block", "type, handler, stack_height")

class VirtualMachine:
    """Only one instance will be created each time the
    program is run. The entry point is the method `run_code`"""

    def __init__(self):
        self.frames: list[Frame] = []  # The call stack of frames
        self.frame: Frame = Any  # The current frame
        self.return_value = Any
        self.last_exception: Tuple[Any, Any, Any] = Any

    def run_code(self, code, global_names=None, local_names=None):
        """An entry point to execute code using the VM"""
        frame = self.make_frame(
            code, global_names=global_names, local_names=local_names
        )
        self.run_frame(frame)

    # FRAME MANIPULATION
    def make_frame(self, code, callargs={}, global_names=None, local_names=None):
        if global_names is not None and local_names is not None:
            local_names = global_names
        elif self.frames:
            global_names = self.frame.global_names
            local_names = {}
        else:
            global_names = local_names = {
                "__builtins__": __builtins__,
                "__name__": "__main__",
                "__doc__": None,
                "__package__": None,
            }

        local_names.update(callargs)
        frame = Frame(code, global_names, local_names, self.frame)
        return frame

    def push_frame(self, frame):
        self.frames.append(frame)
        self.frame = frame

    def pop_frame(self):
        self.frames.pop()
        if self.frames:
            self.frame = self.frames[-1]
        else:
            self.frame = None

    def run_frame(self, frame):
        """Run a frame until it returns (somehow).
        Exceptions are raised, the return value is returned"""
        self.push_frame(frame)
        while True:
            byte_name, arguments = self.parse_byte_and_args()

            why = self.dispatch(byte_name, arguments)

            # Deal with any block management we need to do
            while why and frame.block_stack:
                why = self.manage_block_stack(why)

            if why:
                break
        self.pop_frame()

        if why == "exception":
            exc, val, tb = self.last_exception
            err = exc(val)
            err.__traceback__ = tb
            raise err
        return self.return_value

    # DATA STACK MANIPULATION
    def top(self):
        return self.frame.stack[-1]

    def pop(self):
        return self.frame.stack.pop()

    def push(self, *vals):
        self.frame.stack.extend(vals)

    def popn(self, n):
        """Pop a number of values from the value stack.
        A list of `n` values is returned the deepest value
        first"""
        if n is not 0:
            ret = self.frame.stack[-n:]
            self.frame.stack[-n:] = []
            return ret
        return []

    def parse_byte_and_args(self):
        """The meaning of the argument to each instruction
        depends on which instruction it is"""
        frame = self.frame
        op_offset = frame.last_instruction
        byte_code = frame.code_obj.co_code[op_offset]
        frame.last_instruction += 1
        byte_name = dis.opname[byte_code]
        if byte_code >= dis.HAVE_ARGUMENT:
            # index into the bytecode
            arg = frame.code_obj.co_code[
                frame.last_instruction : frame.last_instruction + 2
            ]
            frame.last_instruction += 2  # advance the instruction pointer
            arg_val = arg[0] + (arg[1] * 256)
            if byte_code in dis.hasconst:  # look up a constant
                arg = frame.code_obj.co_consts[arg_val]
            elif byte_code in dis.hasname:  # look up a name
                arg = frame.code_obj.co_name[arg_val]
            elif byte_code in dis.haslocal:  # look up a local name
                arg = frame.code_obj.co_varnames[arg_val]
            elif byte_code in dis.hasjrel:  # calculate a relative jump
                arg = frame.last_instruction + arg_val
            else:
                arg = arg_val
            argument = [arg]
        else:
            argument = []
        return byte_name, argument

    def dispatch(self, byte_name, argument):
        """Dispatch by bytename to the corresponding
        methods. Exceptions are caught and set on the
        virtual machine."""
        why = None
        try:
            bytecode_fn = getattr(self, f"byte_{byte_name}", None)
            if bytecode_fn is None:
                if byte_name.startwith("UNARY_"):
                    self.unary_operator(byte_name[6:])
                elif byte_name.startwith("BINARY_"):
                    self.binary_operator(byte_name[6:])
                else:
                    raise VirtualMachineError(f"unsupported bytecode type: {byte_name}")
            else:
                why = bytecode_fn(*argument)
        except Exception as err:
            # deal with exceptions encountered while
            # executing the op
            self.last_exception = sys.exc_info()[:2] + (None,)
            why = "exception"
        return why

    # BLOCK STACK MANIPULATION
    def push_block(self, b_type, handler=None):
        stack_height = len(self.frame.stack)
<<<<<<< HEAD
        self.frame.block_stack.append(Block(b_type, handler, stack_height))

    def pop_block(self):
        return self.frame.block_stack.pop()

    def unwind_block(self, block):
        """Unwind the values on the data stack corresponding
        to a given block"""
        if block.type == "except-handler":
            offset = 3
        else:
            offset = 0

        while len(self.frame.stack) > block.level + offset:
            self.pop()

        if block.type == "except-handler":
            traceback, value, exctype = self.popn(3)
            self.last_exception = exctype, value, traceback

        def manage_block_stack(self, why):
            frame = self.frame
            block = frame.block_stack[-1]
            if block.type == "loop" and why == "continue":
                self.jump(self.return_value)
                why = None
                return why

            self.pop_block()
            self.unwind_block(block)

            if block.type == "loop" and why == "break":
                why = None
                self.jump(block.handler)
                return why


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
