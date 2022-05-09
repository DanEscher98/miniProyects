#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/ioctl.h>
#include <string.h>
#include <unistd.h>
#include <termio.h>
#include <getopt.h>
#include <stdbool.h>

#define nl "\n"

struct options {
	bool newline;
	int times;
	char *dev_name;
	char *cmd;
} options;


void print_help(char *prog_name) {
	printf("Usage: %s [-n] DEVNAME COMMAND\n", prog_name);
	printf("Usage: '-n' optional argument to don't output a new line.\n");
	printf("Usage: '-d' necessary argument to define the output device.\n");
	printf("Usage: '-h' show this message.\n");
	printf("Usage: Will require 'sudo' to run if the executable is not setuid root\n");
	exit(1);
}

void init_options(int argc, char *argv[argc]) {
	int option;
	options.newline = true;
	options.times = 1;
	options.dev_name = NULL;
	options.cmd = NULL;

	while (( option = getopt(argc, argv, "c:d:t:hn") ) != -1) {
		switch (option) {
			case 'c':
				options.cmd = (char*)optarg;
				break;
			case 'd': 
				options.dev_name = optarg;
				break;
			case 't':
				options.times = strtoul(optarg, NULL, 10);
			case 'n':
				options.newline = false;
				break;
			case 'h':
				print_help(argv[0]);
			default:
				printf("Invalid option\n");
				exit(1);
		}
	}
}

int get_fd() {
	int fd;
	if((fd = open(options.dev_name, O_RDWR)) == -1) {
        perror("open DEVICE");
		goto EXCEPTION;
    }

	if (!isatty(fd)) {
		perror("File is not a tty");
		goto EXCEPTION;
	}
	return fd;
EXCEPTION:	
	close(fd);
	exit(1);
}

void set_config(int *serial_port)
{
    struct termios tty;

    tty.c_lflag |= ICANON; /* Enable canonical mode */
    tty.c_lflag |= ECHO; /* Enable the echo */
    tty.c_iflag &= ~INLCR; /* Not translate NL to CR on input*/
    tty.c_iflag |= ICRNL; /* Translate CR to NL on intput */
    tty.c_iflag &= ~IGNCR; /* Not ignore CR */

    /* 115200 8N1*/
    tty.c_cflag &= ~PARENB;
    tty.c_cflag &= ~CSTOPB;
    tty.c_cflag &= ~CSIZE;
    tty.c_cflag |= CS8;
    tty.c_cflag &= ~CRTSCTS;
    cfsetspeed(&tty, B115200);

    if (tcsetattr(*serial_port, TCSANOW, &tty) != 0)
    {
        printf("Error %i: %s\n", errno, strerror(errno));
    }
}

int main (int argc, char *argv[argc]) {
	init_options(argc, argv);
	int fd = get_fd();
	// Open device
    	// Write cmd to device
	struct termios old_termios;
	tcgetattr(fd, &old_termios);
	set_config(&fd);

	if (!options.newline)
        tcdrain(fd);

	for (size_t i=0; i< options.times; i++) {
		for ( int i = 0; options.cmd[i]; i++)
			ioctl(fd, TIOCSTI, options.cmd+i);

		if (options.newline) ioctl(fd, TIOCSTI, nl);
	}

	tcsetattr(fd, TCSANOW, &old_termios);
    close(fd);
	return EXIT_SUCCESS;
}
