# Mutacoes (cx08_13), com as 8 ligas ate 2002-06-01:
# - o texto de quem passa da temporada regular vira "through to final" (o comparador de texto acusa com o modo
#   CM_TEXTO_NOTICIAS);
# - nas finais o terceiro fica com o codigo 6 (o comparador do progresso acusa);
# - os jogos das finais levam 4 reservas (o comparador do banco acusa; as escalacoes mudam).
MUTACOES = [
    ('cx08_13_texto_final',
     '        0xff => 0x987c88, // "{club} through to playoff phase"',
     '        0xff => 0x987c58, // mutation',
     'cx08_13 australian qualify text'),
    ('cx08_13_terceiro_codigo_6',
     '(code(current, [1, 1, 5, 7]), code(best, [1, 1, 5, 7]), code(worst, [3, 3, 5, 7]))',
     '(code(current, [1, 1, 6, 7]), code(best, [1, 1, 5, 7]), code(worst, [3, 3, 5, 7])) // mutation',
     'cx08_13 australian finals progress'),
    ('cx08_13_finais_4_reservas',
     'return fixture.set_squad_rules((fixture.squad_rules() & 0xff35) | 0x35);',
     'return fixture.set_squad_rules((fixture.squad_rules() & 0xff34) | 0x34); // mutation',
     'cx08_13 australian stage bench'),
]
