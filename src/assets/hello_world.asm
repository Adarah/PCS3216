 @ /F00

START
    LD  LDA0
    MM  LDA
    LD  LDA0+1
    MM  LDA+1

LDA
    K   0
    K   0
    PD  0
    LD  LDA+1
    +   ONE
    MM  LDA+1
    JZ  CARRY

CHECK_IF_DONE
    LD  LEN
    -   ONE
    MM  LEN
    JZ  END
    JP  LDA

CARRY
    LD  LDA
    +   ONE
    MM  LDA
    JP  CHECK_IF_DONE


END
    HM  0


@ /100
LEN     K   12
ONE     K   1
COUNT   K   0

LDA0    LD  DATA

@ /200
DATA
    K   "H"
    K   "e"
    K   "l"
    K   "1"
    K   "o"
    K   ","
    K   32                      ; Program breaks if you try to use a space character
    K   "w"
    K   "o"
    K   "r"
    K   "l"
    K   "d"
    # START
