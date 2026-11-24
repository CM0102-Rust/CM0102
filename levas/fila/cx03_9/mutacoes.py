# Mutacoes (cx03_9), carreira com as ligas da Escocia e de Gales ate 2002-08-01. Nenhuma abre caixa. As noticias so
# a comparacao acusa (o texto nao volta para a simulacao; modo CM_TEXTO_NOTICIAS); o progresso, o comparador.
# - a Copa da Liga galesa com o mata-mata na fase 4: a fase 4 dos grupos passa a dar o texto do confronto;
# - o texto da Copa da Premier galesa com o formato "through to semi finals" (0x9a7dac): o texto muda;
# - o 3o lugar da Copa da Liga vale 5 no atual e no melhor: o comparador do progresso acusa.
MUTACOES = [
    ('cx03_9_liga_galesa_mata_mata_na_fase_4',
     'news, text, 5, 0x9a7e9c, "cx03_9 welsh league cup through to quarter finals")',
     'news, text, 4, 0x9a7e9c, "cx03_9 welsh league cup through to quarter finals") // mutation'),
    ('cx03_9_premier_galesa_texto_das_semifinais',
     'news, text, 1, 0x9a7e9c, "cx03_9 welsh premier cup through to quarter finals")',
     'news, text, 1, 0x9a7dac, "cx03_9 welsh premier cup through to quarter finals") // mutation'),
    ('cx03_9_terceiro_lugar_vale_cinco',
     '            1 | 2 => 1,\n            3 => 9,\n            _ => code,',
     '            1 | 2 => 1,\n            3 => 5, // mutation\n            _ => code,'),
]
