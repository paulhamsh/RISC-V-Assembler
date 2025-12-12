start:
    lw   x3, x2(0)            
    lw   x1, x2(4)            
    add  x2, x3, x1           
    sw   x2, x1(0)            
    add  x2, x3, x1           
    addi x2, x3  20              
    sll  x2, x3, x1           
    sltu x2, x3, x1           
    and  x2, x3, x1           
    or   x2, x3, x1           
    slt  x2, x3, x1           
    add  x3, x3, x3           
    beq  x3, x2, 8            // {jump 60}
    bne  x3, x2, 4            // {jump 60}
    jal  x0, start                
    jal  x0, -48              // {jump 16}

