label1:
          and   x1, x1, x0
          lw    x2, 4(x1)
          beq   x1, x2, label2
          sw    x1, 4(x2)
label2:
          jal   x0, label1
