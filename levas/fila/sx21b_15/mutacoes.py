# Mutacoes (sx21b_15). Roda na partida do jogo; cai pelo comparador (o valor).
# - a marca invalida vira 0xf2fffff2: o comparador acusa na primeira leitura (o indice, a lingua). No modo Rust (a
#   triagem) o jogo leria o arquivo como do formato B: se travar a partida, a triagem a pega pelo limite de tempo.
MUTACOES = [
    ('sx21b_15_marca_b',
     '        MARKER_INVALID // sx21b_15: no marker',
     '        MARKER_B // mutation: always format B',
     'sx21b_15 format marker found'),
]
