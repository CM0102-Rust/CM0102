# Mutacoes (cx09_8) pela carreira com a Belgica ate 2002-08-01 (o campeao de cada periodo vai a repescagem de acesso
# e ganha a noticia; a repescagem e a virada mudam quem sobe):
# - o terceiro periodo comeca na rodada 21 (e nao na 20): as tabelas do segundo e do terceiro e os campeoes mudam;
# - os jogos do grupo B nao contam nos periodos: o grupo B fica sem campeoes de periodo (sem noticia nem repescagem);
# - a ordem das fases inverte o periodo: o comparador da ordem acusa (e as fases trocam de indice).
MUTACOES = [
    ('cx09_8_periodo_rodada_21',
     '    } else if round < 20 {\n        2\n',
     '    } else if round < 21 {\n        2\n'),
    ('cx09_8_grupo_b_sem_periodos',
     '    if stage != 0xff && stage != 0 {\n        return 1;',
     '    if stage != 0xff { // mutation\n        return 1;'),
    ('cx09_8_ordem_invertida',
     '        return ga as i16 as i32 - gb as i16 as i32;\n    }\n    pa as i32 - pb as i32\n',
     '        return ga as i16 as i32 - gb as i16 as i32;\n    }\n    pb as i32 - pa as i32 // mutation\n'),
]
