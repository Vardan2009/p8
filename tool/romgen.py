def write_logisim_hex(mem_dict, out, compress_zeros=True):
    mem_size = max(int(addr, 2) for addr in mem_dict) + 1
    memory = [0] * mem_size

    for addr_bin, val_bin in mem_dict.items():
        addr = int(addr_bin, 2)
        val = int(val_bin, 2)
        memory[addr] = val

    print("v2.0 raw", file=out)

    if compress_zeros:
        i = 0
        while i < len(memory):
            if memory[i] == 0:
                count = 1
                while i + count < len(memory) and memory[i + count] == 0:
                    count += 1
                print(f"{count}*0", end=" ", file=out)
                i += count
            else:
                print(f"{memory[i]:02X}", end=" ", file=out)
                i += 1
    else:
        for val in memory:
            print(f"{val:02X}", end=" ", file=out)

    print(file=out)


if __name__ == "__main__":
    memory = {
        # NOOP 0000 Instruction (Not based on Z or N flags)
        "00000000": "00000000",
        "00000001": "00000000",
        "00000010": "00000000",
        "00000011": "00000000",
        # MOV 0001 Instruction (Not based on Z or N flags)
        "00000100": "11000000",
        "00000101": "11000000",
        "00000110": "11000000",
        "00000111": "11000000",
        # LDI 0010 Instruction (Not based on Z or N flags)
        "00001000": "10000000",
        "00001001": "10000000",
        "00001010": "10000000",
        "00001011": "10000000",
        # LDR 0011 Instruction (Not based on Z or N flags)
        "00001100": "11100000",
        "00001101": "11100000",
        "00001110": "11100000",
        "00001111": "11100000",
        # STR 0100 Instruction (Not based on Z or N flags)
        "00010000": "00000100",
        "00010001": "00000100",
        "00010010": "00000100",
        "00010011": "00000100",
        # ADD 1000 Instruction (Not based on Z or N flags)
        "00100000": "10110000",
        "00100001": "10110000",
        "00100010": "10110000",
        "00100011": "10110000",
        # ADDI 1001 Instruction (Not based on Z or N flags)
        "00100100": "10100000",
        "00100101": "10100000",
        "00100110": "10100000",
        "00100111": "10100000",
        # SUB 1010 Instruction (Not based on Z or N flags)
        "00101000": "10110000",
        "00101001": "10110000",
        "00101010": "10110000",
        "00101011": "10110000",
        # AND 1011 Instruction (Not based on Z or N flags)
        "00101100": "10110000",
        "00101101": "10110000",
        "00101110": "10110000",
        "00101111": "10110000",
        # OR 1100 Instruction (Not based on Z or N flags)
        "00110000": "10110000",
        "00110001": "10110000",
        "00110010": "10110000",
        "00110011": "10110000",
        # XOR 1101 Instruction (Not based on Z or N flags)
        "00110100": "10110000",
        "00110101": "10110000",
        "00110110": "10110000",
        "00110111": "10110000",
        # NOT 1110 Instruction (Not based on Z or N flags)
        "00111000": "10110000",
        "00111001": "10110000",
        "00111010": "10110000",
        "00111011": "10110000",
        # JMP 0101 Instruction (Not based on Z or N flags)
        "00010100": "00000010",
        "00010101": "00000010",
        "00010110": "00000010",
        "00010111": "00000010",
        # JZ 0110 Instruction
        "00011000": "00000000",
        "00011001": "00000000",
        "00011010": "00000010",
        "00011011": "00000010",
        # JN 0111 Instruction
        "00011100": "00000000",
        "00011101": "00000010",
        "00011110": "00000000",
        "00011111": "00000010",
    }

    with open("controlrom.hex", "w") as f:
        write_logisim_hex(memory, f)