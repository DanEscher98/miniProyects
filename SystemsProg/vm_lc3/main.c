#include <signal.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

// REGISTERS
enum {
	R_R0 = 0,
	R_R1,
	R_R2,
	R_R3,
	R_R4,
	R_R5,
	R_R6,
	R_R7,
	R_PC, // program counter
	R_COND,
	R_COUNT
};

// INSTRUCTION SET
enum {
	OP_BR = 0, // branch
	OP_JPR,	   // jump register
	OP_LOAD,   // load
	OP_ST,	   // store
	OP_LDR,	   // load register
	OP_STR,	   // store register
	OP_LDI,	   // load indirect
	OP_STI,	   // store indirect
	OP_FREE,   // unused
	OP_NOT,	   // bitwise not
	OP_AND,	   // bitwise and
	OP_ADD,	   // add
	OP_JUMP,   // jump
	OP_RES,	   // reserved
	OP_LEA,	   // load effective address
	OP_TRAP	   // execute trap
};

// CONDITION FLAGS
enum { FL_POS = 1 << 0, FL_ZERO = 1 << 1, FL_NEG = 1 << 2 };

uint16_t memory[UINT16_MAX];
uint16_t reg[R_COUNT];

int main(int argc, const char** argv)
{
	// LOAD ARGUMENTS
	if (argc < 2) {
		/* show usage string */
		printf("lc3 [image-file] ...\n");
		exit(2);
	}
	for (int i = 1; i < argc; i++) {
		if (!read_image(argv[i])) {
			printf("failed to load image: %s\n", argv[i]);
			exit(1);
		}
	}

	// SETUP

	/* set the PC to starting position */
	/* 0x3000 is the default */
	enum { PC_START = 0x3000 };
	reg[R_PC] = PC_START;

	bool running = true;
	while (running) {
		/* FETCH */
		uint16_t instr = mem_read(reg[R_PC]++);
		uint16_t op = instr >> 12;

		switch (op) {
		case OP_BR: { /* branch */
			break;
		}
		case OP_JPR: { /* jump register */
			break;
		}
		case OP_LOAD: { /* load */
			break;
		}
		case OP_ST: { /* store */
			break;
		}
		case OP_LDR: { /* load register */
			break;
		}
		case OP_STR: { /* store register */
			break;
		}
		case OP_LDI: { /* load indirect */
			break;
		}
		case OP_STI: { /* store indirect */
			break;
		}
		case OP_FREE: { /* unused */
			break;
		}
		case OP_NOT: { /* bitwise not */
			break;
		}
		case OP_AND: { /* bitwise and */
			break;
		}
		case OP_ADD: { /* add */
			break;
		}
		case OP_JUMP: { /* jump */
			break;
		}
		case OP_RES: { /* reserved */
			break;
		}
		case OP_LEA: { /* load effective address */
			break;
		}
		case OP_TRAP: { /* trap */
			break;
		}
		default: {
			// BAD OPCODE
			break;
		}
		}
	}

	// SHUTDOWN
	return EXIT_SUCCESS;
}
