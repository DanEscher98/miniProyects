#!/usr/bin/python3
import fcntl
import logging
import os
import sys
import termios
from os import O_RDWR, listdir, ttyname
from os.path import join
from termios import (B9600, CLOCAL, CREAD, CS8, CSIZE, CSTOPB, ECHO, ICANON,
                     ISIG, NCCS, PARENB, VMIN, VTIME)

tty_path = ttyname(sys.stdout.fileno())


def get_pts():
    """Get available pseudo-terminals"""
    dev_path = "/dev/pts"
    openpts = []
    for f in listdir(dev_path):
        fullpath = join(dev_path, f)
        if f != "ptmx" and fullpath != tty_path:
            openpts.append(fullpath)
    return openpts


def print_termios(fd):
    ts = termios.tcgetattr(fd)
    print(f"tc_iflag: {ts[0]}")
    print(f"tc_oflag: {ts[1]}")
    print(f"tc_cflag: {ts[2]}")
    print(f"tc_lflag: {ts[3]}")


def init_stdio():
    stdio = [0, 0, 0, 0, 0, 0, [0] * NCCS]
    stdio[0] = 0  # iflag
    stdio[1] = 0  # oflag
    stdio[2] = (
        (((CREAD | CLOCAL) & ~CSIZE) | ~CS8)
        & ~PARENB
        & ~CSTOPB
        & ~(ICANON | ECHO | ISIG)
    )  # cflag
    stdio[3] = 0  # lflag
    stdio[4] = B9600  # ispeed
    stdio[5] = B9600  # ospeed
    stdio[6][VMIN] = 1  # Minimum number of characters for noncanonical read
    stdio[6][VTIME] = 5  # Timeout in deciseconds for noncanonical read (TIME)
    return stdio


def no_echo():
    stdio = [0, 0, 0, 0, 0, 0, [0] * NCCS]
    stdio[0] = 0  # iflag
    stdio[1] = 0  # oflag
    stdio[2] = ~ECHO  # cflag
    stdio[3] = 0  # lflag
    stdio[4] = B9600  # ispeed
    stdio[5] = B9600  # ospeed
    stdio[6][VMIN] = 1  # Minimum number of characters for noncanonical read
    stdio[6][VTIME] = 5  # Timeout in deciseconds for noncanonical read (TIME)
    return stdio


def prompt_noecho(prompt="Password: "):
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        termios.tcsetattr(fd, termios.TCSADRAIN, no_echo())
        passwd = input(prompt)
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)
    print(f"Your password is {passwd}")
    # sys.stdout.flush()


def initi_serial():
    serial_data = "hello world"
    filepath = sys.argv[1]
    with os.fdopen(os.open(filepath, O_RDWR), "w") as fd:
        old = termios.tcgetattr(fd)
        try:
            termios.tcsetattr(fd, termios.TCSANOW, init_stdio())
            for _ in range(5):
                fd.write(serial_data)
        finally:
            termios.tcsetattr(fd, termios.TCSANOW, old)
    logging.info("Program finished")


def send_message(msg):
    for file in get_pts():
        with open(file, "wb+", buffering=0) as pts:
            pts.write(f"Hello {file} from {tty_path}\n".encode())
            pts.write(msg)
            pts.flush()


if __name__ == "__main__":
    prompt_noecho()
