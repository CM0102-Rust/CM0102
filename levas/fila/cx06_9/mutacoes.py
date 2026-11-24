# Mutacoes (cx06_9), com as 8 ligas e ATE 2003-08-15. O texto nao volta a simulacao: as duas sao acusadas pelo
# comparador de texto (com o modo CM_TEXTO_NOTICIAS), nao pela carreira.
# - quem passa do grupo ganha o texto da Oceania ("qualify for semi final", 0xa6a0ac);
# - o terceiro colocado ganha o texto do quarto.
MUTACOES = [
    ('cx06_9_texto_do_grupo',
     'a8, news, text, 1, None, 0x9c74b4, &finals);',
     'a8, news, text, 1, None, 0xa6a0ac, &finals); // mutation',
     'cx06_9 confed fate text'),
    ('cx06_9_terceiro_como_quarto',
     '        3 => 0x987264, // finish third place in',
     '        3 => 0x987200, // mutation',
     'cx06_9 confed fate text'),
]
