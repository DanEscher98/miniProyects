#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
// Unix specific
#include <fcntl.h>
#include <unistd.h>

#include <sys/mman.h>
#include <sys/termios.h>
#include <sys/time.h>
#include <sys/types.h>
// Windows specific
//#include <Windows.h>
//#include <conio.h>

// Platform Specifics (Unix)

u_int16_t check_key()
{
	fd_set readfds;
	FD_ZERO(&readfds);
	FD_SET(STDIN_FILENO, &readfds);

	struct timeval timeout;
	timeout.tv_sec = 0;
	timeout.tv_usec = 0;
	return select(1, &readfds, NULL, NULL, &timeout) != 0;
}

struct termios original_tio;

void disable_input_buffering()
{
	tcgetattr(STDERR_FILENO, &original_tio);
	struct termios new_tio = original_tio;
	new_tio.c_cflag &= ~ICANON & ~ECHO;
	tcsetattr(STDIN_FILENO, TCSANOW, &new_tio);
}

void restore_input_buffering()
{
	tcsetattr(STDIN_FILENO, TCSANOW, &original_tio);
}

void handle_interrupt(int signal)
{
	restore_input_buffering();
	printf("\n");
	exit(-2);
}

// Platform Specific (Windows)
//
// HANDLE hStdin = INVALID_HANDLE_VALUE;
//
// u_int16_t check_key(){
// 	return WaitForSingleObject(hStdin, 1000) == WAIT_OBJECT_0 && _kbhit();
// }
//
// DWORD fdwMode, fdwOldMode;
//
// void disable_input_buffering()
// {
//		hStdin = GetStdHandle(STD_INPUT_HANDLE);
//		GetConsoleMode(hStdin, &fdwOldMode); // save old mode
//		fdwMode = fdwOldMode
//				^ ENABLE_ECHO_INPUT		// no input echo
//				^ ENABLE_LINE_INPUT;	// return when one or
//										// more characters are available
//		SetConsoleMode(hStdin, fdwMode);// set new mode
//		FlushConsoleInputBuffer(hStdin);// clear buffer
// }
//
// void restore_input_buffering()
// {
//		SetConsoleMode(hStdin, fdwOldMode);
// }
