#include "lib.h"
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
typedef enum {
	OP_BR = 0,	  // branch
	OP_ADD = 1,	  // add
	OP_LOAD = 2,  // load
	OP_ST = 3,	  // store
	OP_LDR = 6,	  // load register
	OP_STR = 7,	  // store register
	OP_LDI = 10,  // load indirect
	OP_STI = 11,  // store indirect
	OP_FREE = 8,  // unused
	OP_NOT = 9,	  // bitwise not
	OP_AND = 5,	  // bitwise and
	OP_JPR = 4,	  // jump register
	OP_JUMP = 12, // jump
	OP_RES = 13,  // reserved (illegal)
	OP_LEA = 14,  // load effective address
	OP_TRAP = 15  // execute trap
} opcode;

enum {
	MR_KBSR = 0xFE00, // keyboard status
	MR_KBDR = 0xFE02  // keyboard data  // keyboard data
};

// CONDITION FLAGS
enum { FL_POS = 1 << 0, FL_ZERO = 1 << 1, FL_NEG = 1 << 2 };

uint16_t memory[UINT16_MAX]; // 65'535 == 0xFFFF
uint16_t reg[R_COUNT];		 // 10
uint16_t sign_extend(uint16_t, int);
// swap
void update_flags(uint16_t);
// read image file
void read_image_file(FILE*);
uint16_t swap16(uint16_t);
int read_image(const char*);
// check key
// memory access
void mem_write(uint16_t, uint16_t);
uint16_t mem_read(uint16_t);
// input buffering
// handle interrupt
void compute(void);

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

	compute();

	// SHUTDOWN
	return EXIT_SUCCESS;
}

// It extends a number to 16 bits, filling in 0's for positive numbers and 1's
// for negative numbers, so that original values are preserved. Two's Complement
uint16_t sign_extend(uint16_t x, int bit_count)
{
	if ((x >> (bit_count - 1)) & 1) {
		x |= (0xFFFF << bit_count);
	}
	return x;
}

// Any time a value is written to a register, we need to update the flags to
// indicate its sign.
void update_flags(uint16_t r)
{
	if (reg[r] == 0) {
		reg[R_COND] = FL_ZERO;
	} else if (reg[r] >> 15) {
		// a 1 in the left-most bit indicates negative
		reg[R_COND] = FL_NEG;
	} else {
		reg[R_COND] = FL_POS;
	}
}

// LOADING PROGRAMS
void read_image_file(FILE* file)
{
	// the origin tells us where in memory to place the image
	uint16_t origin;
	fread(&origin, sizeof(origin), 1, file);
	origin = swap16(origin);

	// we know the maximum file size so we only need one fread
	uint16_t max_read = UINT32_MAX - origin;
	uint16_t* p = memory + origin;
	size_t read = fread(p, sizeof(uint16_t), max_read, file);

	// swap to little endian
	while (read-- > 0) {
		*p = swap16(*p);
		++p;
	}
}

/* Most of modern computers are little endian, but LC-3 programs are big-endian.
 * So we need to swap each `uint16` that is loaded. */
uint16_t swap16(uint16_t x) { return (x << 8) | (x >> 8); }

int read_image(const char* image_path)
{
	FILE* file = fopen(image_path, "rb");
	if (!file)
		return 0;
	read_image_file(file);
	fclose(file);
	return 1;
}

void mem_write(uint16_t address, uint16_t val) { memory[address] = val; }

uint16_t mem_read(uint16_t address)
{
	if (address == MR_KBSR) {
		if (check_key()) {
			memory[MR_KBSR] = (1 << 15);
			memory[MR_KBDR] = getchar();
		} else {
			memory[MR_KBSR] = 0;
		}
	}
	return memory[address];
}

void compute()
{
	/* set the PC to starting position */
	/* 0x3000 is the default */
	enum { PC_START = 0x3000 };
	reg[R_PC] = PC_START;

	bool running = true;
	while (running) {
		/* FETCH */
		uint16_t instruction = mem_read(reg[R_PC]++);
		uint16_t op = instruction >> 12;

		switch (op) {
		case OP_BR: { /* branch */
			break;
		}
		case OP_ADD: { /* add */
			// destination register (DR)
			uint16_t r0 = (instruction >> 9) & 0x7;
			// first operand (SR1)
			uint16_t r1 = (instruction >> 6) & 0x7;
			// whether we are in immediate mode
			uint16_t imm_flag = (instruction >> 5) & 0x1;

			if (imm_flag) {
				uint16_t imm5 = sign_extend(instruction & 0x1F, 5);
				reg[r0] = reg[r1] + imm5;
			} else {
				uint16_t r2 = instruction & 0x7;
				reg[r0] = reg[r1] + reg[r2];
			}
			update_flags(r0);
		} break;
		case OP_LOAD: { /* load */
			break;
		}
		case OP_ST: { /* store */
			break;
		}
		case OP_JPR: { /* jump register */
			break;
		}
		case OP_AND: { /* bitwise and */
			// destination register (DR)
			uint16_t r0 = (instruction >> 9) & 0x7;
			// first operand (SR1)
			uint16_t r1 = (instruction >> 6) & 0x7;
			// whether we are in immediate mode
			uint16_t imm_flag = (instruction >> 5) & 0x1;

			if (imm_flag) {
				uint16_t imm5 = sign_extend(instruction & 0x1F, 5);
				reg[r0] = reg[r1] + imm5;
			} else {
				uint16_t r2 = instruction & 0x7;
				reg[r0] = reg[r1] + reg[r2];
			}
			update_flags(r0);
		} break;
		case OP_LDR: { /* load register */
			break;
		}
		case OP_STR: { /* store register */
			break;
		}
		case OP_FREE: { /* unused */
			break;
		}
		case OP_NOT: { /* bitwise not */
			break;
		}
		case OP_LDI: { /* load indirect */
			break;
		}
		case OP_STI: { /* store indirect */
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
}
