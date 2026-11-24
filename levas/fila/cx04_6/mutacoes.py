# Mutacoes (cx04_6), com as 8 ligas ate 2003-08-15 (o Mundial de Clubes de 2003):
# - o pior codigo do primeiro do grupo fica 5 no lugar de 4: o comparador do progresso acusa;
# - o terceiro colocado ganha o texto do vice: o comparador de texto acusa com o modo CM_TEXTO_NOTICIAS.
MUTACOES = [
    ('cx04_6_mundial_pior_codigo',
     '[[1, 5, 7, 0xa], [1, 1, 7, 0xa], [4, 5, 7, 0xa]]',
     '[[1, 5, 7, 0xa], [1, 1, 7, 0xa], [5, 5, 7, 0xa]] /* mutation */',
     'cx04_6 club world championship progress'),
    ('cx04_6_mundial_terceiro_vice',
     '(2, 0x9872c8), (3, 0xad57a8)]; // "win"',
     '(2, 0x9872c8), (3, 0x9872c8)]; // mutation "win"',
     'cx04_6 club world championship fate text'),
]
