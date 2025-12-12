// comment line
start:
another_label:
another:
// with comment
    lw   x3, x2(0)            
    lw   x1, x2[1]   
jump_point:
here:
    add  x2, x3, x1           
    jal  x0, -8                 
    sw   x2, x1 + 0            
    xor  x2, x3, x1           
    addi x2  x3  12               
    sll  x2  x3  x1           
    slt  x2, x3, x1           
    and  x2, x3, x1           
    or   x2, x3, x1           
    sltu x2, x3, x1           
    add  x3, x3, x3           
    beq  x3, x2, 4            // {jump 60}
    bne  x3, x2, 0            // {jump 60}
    jal  x0, another              
cont:
    lw   x1, x2(5)            // comment
    sw   x3, x5(3)            // this is
    add  x4, x2, x1           // a
