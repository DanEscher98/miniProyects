#define  _GNU_SOURCE
#include  <termios.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdarg.h>

int get_fd(char*, struct termios*);
int read_proc(int, char*, size_t);
int write_proc(int, char*, size_t);

int main(int argc, char* argv[]) {
	struct termios oldts;
	int fd = get_fd(argv[1], &oldts);

	char bufout = 'A';
	char bufin;

	while(true) {
		// WRITE_PROC
		if(write_proc(fd, &bufout, 1) == -1) break;

		// READ procedure
		if(read_proc(fd, &bufin, 1) == -1) break;

		tcdrain(fd);
		sleep(2);
	}

	tcsetattr(fd, TCSAFLUSH, &oldts);
	close(fd);
	return  EXIT_SUCCESS;
}

int write_proc(int fd, char *bufout, size_t nbytes) {
	if(*bufout == 'A') *bufout = 'B';
	else if(*bufout == 'B') *bufout = 'A';
	printf("writing %c\n", *bufout);
	return write(fd, &bufout, 1);
}

int read_proc(int fd, char *bufin, size_t nbytes) {
	int c;
	printf("reading\n");
	c = read(fd, &bufin, 1);
	printf("read %i bytes: %c %d\n", c, (char)*bufin, *bufin);
	return c;
}

int get_fd(char *dev_name, struct termios *oldts) {
	int fd;
	if ((fd = open(dev_name, O_RDWR | O_NOCTTY)) < 0) {
		perror("open");
		exit(1);
	}
	if (isatty(fd) != 1) {
		perror("The fd isn't a tty");
		exit(1);
	}
	if(tcgetattr(fd, oldts)) {
	  perror("tcgetattr");
	  exit(1);
	}
	// RAW
	struct termios newts = *oldts;
	cfmakeraw(&newts);
	tcsetattr (fd, TCSANOW, &newts);

	return fd;
}
