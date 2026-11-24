# Mutacoes (cx01_8), carreira com a Argentina ate 2002-08-01. Nenhuma abre caixa.
# - a media menor primeiro: a ordem das medias do rebaixamento inverte (quem cai na primeira argentina muda) e o
#   comparador acusa;
# - os pontos da temporada, menos primeiro: o mata-mata do acesso da segunda muda e o comparador acusa.
MUTACOES = [
    ('cx01_8_media_menor_primeiro',
     '    if x > y {\n        return -1;\n    }\n    if !(x >= y) {\n        return 1;\n    }',
     '    if x < y { // mutation\n        return -1;\n    }\n    if !(x <= y) {\n        return 1;\n    }'),
    ('cx01_8_pontos_menos_primeiro',
     '    let (x, y) = (read_i16(a + column), read_i16(b + column));\n    if x > y {',
     '    let (x, y) = (read_i16(b + column), read_i16(a + column)); // mutation\n    if x > y {'),
]
