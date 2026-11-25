# Mutacoes (rx12_16), pela carreira com a Italia na base NORMAL ate 2002-09-30 (a virada italiana do verao de 2002 e
# a C2 de 2002/03 que comeca em seguida).
# - sobem oito clubes da Serie D, nao nove (outro clube fica na C2 e muda os grupos e o calendario);
# - o reagrupamento de 17 em 17 (os grupos mudam).
MUTACOES = [
    ('rx12_16_oito_sobem',
     '        for t in 0..9 { // rx12_16: the nine promoted',
     '        for t in 0..8 { // mutation',
     'rx12_16 serie d up to c2'),
    ('rx12_16_grupos_de_17',
     '        let group = if k < 0x12 { records[2] } else if k < 0x24 { records[1] } else { records[0] }; // rx12_16: C2 C, C2 B, C2 A',
     '        let group = if k < 0x11 { records[2] } else if k < 0x24 { records[1] } else { records[0] }; // mutation',
     'rx12_16 serie c2 regrouping'),
]
