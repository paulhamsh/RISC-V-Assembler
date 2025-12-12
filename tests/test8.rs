interim:
start:
    sub  x7, x3, x3           // comment
    sll  x6, x4, x1           
    andi x5, x5, 10         
    slt  x4, x6, x3           
    and  x3, x7, x4           
    or   x2, x5, x6           
    add  x1, x2, x3               
    beq  x1, x2, 4
    beq  x1, x2, -8      
    bne  x1, x2, end   
    lui  x5, x5, 103
    jal  x0,interim              
end:
    jal  x0, end                  
    jal  x0 start
