# Mutacoes (cqy_1). O calendario da Serie C roda na criacao da carreira de 8 ligas (os grupos da primeira fase) e quando
# cada fase chega (contadores "cqy_1 serie c ..."); as datas sao levadas ao dia da semana da rodada:
# - a 2a rodada dos grupos da primeira fase com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - a 2a rodada dos grupos da terceira fase com o dia da semana 5 no lugar de 6;
# - a final uma semana depois (16/nov no lugar de 9/nov).
MUTACOES = [
    ('cqy1_serie_c_primeira_fase_segunda_rodada_um_dia_antes',
     'Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 7, 0, 6, 1, 0),',
     'Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 7, 0, 5, 1, 0) /* mutation: one weekday earlier */,',
     'cqy_1 serie c first round'),
    ('cqy1_serie_c_terceira_fase_segunda_rodada_um_dia_antes',
     'Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 4, 0xa, 0, 6, 1, 0),',
     'Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 4, 0xa, 0, 5, 1, 0) /* mutation: one weekday earlier */,',
     'cqy_1 serie c third round'),
    ('cqy1_serie_c_final_uma_semana_depois',
     'Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 9, 0xb, 0, 6, 1, 0), Cqp11W(0x7, 0x96)',
     'Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 0x10, 0xb, 0, 6, 1, 0) /* mutation: a week later */, Cqp11W(0x7, 0x96)',
     'cqy_1 serie c final'),
]
