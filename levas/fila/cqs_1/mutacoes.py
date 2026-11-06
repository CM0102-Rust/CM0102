# Mutacoes (cqs_1). As tres rodam na criacao da carreira de 8 ligas (a MLS de 2001) e na virada para 2002:
# - a tabela da primeira divisao com as linhas de baixo para cima: outra ordem da tabela (os jogos e o desempate do
#   qsort do play-off partem dela);
# - a segunda divisao com a vitoria valendo o empate (+0xc3 no lugar de +0xc2): outra tabela da fase 0, e os pontos dela
#   decidem quem vai ao play-off e com quem;
# - a terceira divisao deixando a fase atual em 0: a fase seguinte so anda de 1 para 2, e a MLS de 2001 fica sem
#   play-off (outros jogos em setembro e outubro).
MUTACOES = [
    ('cqs1_tabela_de_baixo_para_cima',
     'iniciar(this, n as u8 as u32, c, 0, 0); // cqs1: a linha seguinte da primeira divisão',
     'iniciar(this, (3 - n) as u8 as u32, c, 0, 0); // mutacao: as linhas de baixo para cima',
     'cqs_1 tabela da mls'),
    ('cqs1_segunda_vitoria_vale_empate',
     'cqs1_divisao(this, 0x47b, 0, 0, 0xc2,',
     'cqs1_divisao(this, 0x47b, 0, 0, 0xc3, /* mutacao: a vitoria vale o empate */',
     'cqs_1 segunda divisao'),
    ('cqs1_terceira_sem_fase_atual',
     'cqs1_divisao(this, 0x414, 1, 1, 0xc2,',
     'cqs1_divisao(this, 0x414, 1, 0, 0xc2, /* mutacao: a fase atual fica 0 */',
     'cqs_1 terceira divisao'),
]
