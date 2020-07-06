@   0
;; Os 6 primeiros bytes sao metadados.
;; O byte 1 eh o numero de fitas
;; Os bytes 2 e 3 sao o endereco do inicio do programa
;; Os 4 e 5 sao o endereco onde a fita deve ser montada
;; O ultimo eh o comprimento da fita
INIT
    GD  0
    MM  NUM_FITAS
    GD  0
    MM  DONE
    GD  0
    MM  DONE+1                  ; 10

START
    GD  0
    +   NINETY
    MM  LDA
    GD  0
    MM  LDA+1
    GD  0
    MM  LEN_FITA                ; 24

LOOP
    GD  0
LDA
    K   0                       ; Executes a MM to the target memory position
    K   0
    LD  LDA+1
    +   ONE
    MM  LDA+1
    JZ  CARRY                   ; 34

CHECK_FITA_DONE
    LD  LEN_FITA
    -   ONE
    MM  LEN_FITA
    JZ  CHECK_ALL_DONE
    JP  LOOP                     ; 44

CARRY
    LD  LDA
    +   ONE
    MM  LDA
    JP  CHECK_FITA_DONE         ; 52

CHECK_ALL_DONE
    LD  NUM_FITAS
    -   ONE
    JZ  DONE
    MM  NUM_FITAS
    JP  START                   ; 62

DONE
    ;; Will be overwritten by the address of the place where the program should start
    K   0
    K   0

NUM_FITAS   K   0
LEN_FITA    K   0
ONE         K   1
NINETY      K   /90

    #   INIT
