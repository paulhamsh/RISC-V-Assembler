// branches must be even
// and ought to be a multiple of 4 really
// so use even numbers in assembly
    beq   x2, x2(2)            
    bne  x31, x3(4094)        
    blt  x30, x3(2046)        
    bge  x30, x3(2046)        
    bltu x31, x3(-2)         
    bgeu  x1, x1(-2048)     
  
// jal must be even  
// and ought to be a multiple of 4 really
// so use even numbers in assembly
    jal   x1, 2046             
    jal   x1, 524286           
    jal   x1, -524288          
    jal   x1, -2         
      
// jalr can be odd, but when placed 
// into the pc will have the final bit 
// dropped regardless
    jalr  x1, x2(-1)          
    jalr  x1, x2(1)           
    jalr  x1, x2(2047)  
      
    auipc x1, 524287         
    auipc x1, -1             
    lui   x1, 524287           
    lui   x1, -1               
