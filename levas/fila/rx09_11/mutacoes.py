# Mutacoes (rx09_11), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL (acusadas pelos comparadores lado a lado):
# - as posicoes previstas 6k + 2 (a criacao do jogo e a virada);
# - o codigo 7 da repescagem vira 0x3e em vez de 0x3d (novembro de 2001).
MUTACOES = [
    ('rx09_11_previstas_mais_um',
     '|_, k| k.wrapping_mul(6).wrapping_add(1)); // rx09_11: 6k + 1',
     '|_, k| k.wrapping_mul(6).wrapping_add(2)); // mutation: 6k + 2',
     'cx14_3 grouped predicted positions'),
    ('rx09_11_repescagem_0x3e',
     '7 => 0x3d, // rx09_11: the relegation play-off codes',
     '7 => 0x3e, // mutation',
     'rx09_11 swedish second progress'),
]
