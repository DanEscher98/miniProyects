#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdbool.h>

void get_slavepath(int __fd);
int read_proc(int __fd, char *__buf, size_t __nbytes);
int write_proc(int __fd, char *__buf, size_t __nbytes);

int main(int argc, char* argv[argc]) {
	// get the master fd
	int masterfd = open("/dev/ptmx", O_RDWR | O_NOCTTY);
	get_slavepath(masterfd);

	char bufout = 'D';
	char bufin;

	while (true) {
		// READ procedure
		if(read_proc(masterfd, &bufin, 1) == -1) break;

		// WRITE procedure
		if(write_proc(masterfd, &bufout, 1) == -1) break;

		tcdrain(masterfd);
		sleep(1);
	}

	close(masterfd);
	return EXIT_SUCCESS;
}

int read_proc(int fd, char *bufin, size_t nbytes) {
	int c;
	printf("reading\n");
	c = read(fd, bufin, 1);
	printf("read %i bytes: %c %d\n", c, (char)*bufin, *bufin);
	return c;
}

int write_proc(int fd, char *bufout, size_t nbytes) {
	int c;
	if(*bufout == 'D') *bufout = 'E';
	else if(*bufout == 'E') *bufout = 'D';
	printf("writing %c\n", *bufout);
	c = write(fd, bufout, 1);
	printf("wrote %i bytes\n", c);
	return c;
}

void get_slavepath(int masterfd) {
	if(masterfd < 0) {
		perror("getpt");
		exit(1);
	}

	// grant access to the slave
	if(grantpt(masterfd) < 0) {
		perror("grantpt");
		exit(1);
	}

	// unlock the slave
	if(unlockpt(masterfd) < 0) {
		perror("unlockpt");
		exit(1);
	}

	// get the path to the slave
	char slavepath[64];
	if(ptsname_r(masterfd, slavepath, sizeof(slavepath)) < 0) {
		perror("ptsname_r");
		exit(1);
	}

	printf("Using %s\n", slavepath);
}
