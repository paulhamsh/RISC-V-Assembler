// This can be single stepped in a testbench

start:
    lw   x3, 0(x0)
    lw   x1, 4(x0)
    add  x2, x3, x1
    sw   x2, 4(x0)
    sub  x2, x3, x1
    xori x2, x3, -1
    sll  x2, x3, x1
    srl  x2, x3, x1
    and  x2, x3, x1
    or   x2, x3, x1
    slt  x2, x3, x1
    add  x3, x3, x3
    lui  x3, 1
    auipc x3, 1
    beq  x3, x2, 12
    bne  x3, x2, 8
    jalr x1, 16(x0)
    jalr x1, 16(x0)
