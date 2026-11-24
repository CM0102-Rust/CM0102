# Mutacoes (cx04_5), com as 8 ligas:
# - o segundo do grupo da Oceania fica com o codigo atual 4 no lugar de 3: o comparador do progresso acusa (e as
#   estatisticas da competicao mudam);
# - quem passa do grupo da Oceania ganha o texto "classifica-se para a final" (0xad580c): o comparador de texto acusa
#   com o modo CM_TEXTO_NOTICIAS.
MUTACOES = [
    ('cx04_5_oceania_codigo_atual',
     '[[1, 3, 5, 7], [1, 1, 5, 7], [3, 3, 5, 7]]',
     '[[1, 4, 5, 7], [1, 1, 5, 7], [3, 3, 5, 7]] /* mutation */',
     'cx04_5 oceania club progress'),
    ('cx04_5_oceania_texto_semifinal',
     'a8, news, text, 1, Some(1), 0xa6a0ac, &OCEANIA_FINALS);',
     'a8, news, text, 1, Some(1), 0xad580c, &OCEANIA_FINALS); // mutation',
     'cx04_5 group qualify text'),
]
