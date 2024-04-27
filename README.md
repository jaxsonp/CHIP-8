# CHIP-8 Interpreter

A basic CHIP-8 interpreter written in rust. Mostly targets the original COSMAC VIP interpreter.

_Written using help from Tobias Langhoff's [Guide to making a CHIP-8 emulator ](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) and the Wikipedia [CHIP-8 article](https://en.wikipedia.org/wiki/CHIP-8)._

## Assembler

For ease of writing test programs, I slapped together an extremely basic assembler that takes a file of whitespace separated, hex encoded CHIP-8 code and writes it into a .ch8 binary file, ready to be loaded. The assembler will also ignore all lines starting with a '#', allowing for commented code.

### Usage

`$ assembler.py [-h] [-o output] input`

Output will default to `out.ch8` if no file is specified

### Example source

```
1204 1202

# generating random number
C0FF
A300
F033

# drawing number
F265
6301
6401

F029 D345
7305
F129 D345
7305
F229 D345

# infinite loop
1202
```

Corresponding command: `$ assembler.py examples/src/random.asm -o examples/random.ch8`

## Included Examples

There are some example roms in the `/examples/` directory I provided to test and play around with.

### `ibm.ch8`

Simply displays the IBM logo.

Source: <https://github.com/loktar00/chip8/blob/master/roms/IBM%20Logo.ch8>

### `test_opcode.ch8`

Checks functionality of different opcodes.

Source: <https://github.com/corax89/chip8-test-rom>
