#!/bin/python3

import argparse
import os

def main ():
	parser = argparse.ArgumentParser(
		prog=os.path.basename(__file__),
		description="Assembles a text file containing whitespace separated, hex encoded CHIP-8 instructions to a binary .ch8 file.")
	parser.add_argument("input", help="Input file to read from")
	parser.add_argument("-o", "--output", default="out.ch8", help="Output file to write to")
	args = parser.parse_args()

	# reading file
	with open(args.input, "r") as f:
		raw_program = f.readlines()

	# removing comments (lines that start with '#')
	program = ""
	for line in raw_program:
		if line.startswith("#"):
			continue
		program += line

	instructions = [int(x, 16) for x in program.split()]

	# writing to output
	with open(args.output, "wb") as f:
		for instruction in instructions:
			f.write(instruction.to_bytes(2, byteorder="big", signed=False))
	return

if __name__ == "__main__":
	main()