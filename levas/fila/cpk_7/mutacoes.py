# Mutacoes. O historico com o primeiro deslocamento em vez do ultimo (o comparando acusa no primeiro regen; a carreira
# faz regens e grava historicos depois). A formacao com a casa sem a bola tambem no ataque e a formacao com os setores
# trocados (o comparando acusa na primeira chamada; a carreira pelas partidas: as instructions 0x6ddab0 de cada partida
# acertam as posicoes, e as extras de outro setor sao comuns no avanco pelas pontas). Nenhuma na fluencia (quente).
MUTACOES = [
    ('cpk_historico_ultimo', 'sem_ultimo(cpk7_d(this), par, par + 4); // o último deslocamento',
     'sem_ultimo(cpk7_d(this), par, par); // mutacao: o primeiro'),
    ('cpk_formacao_com_bola', 'pitch_cell(b, 1, arg, &mut saida);',
     'pitch_cell(b, 0, arg, &mut saida); // mutacao: sem a bola'),
    ('cpk_formacao_setor', 'if (setor_extra as i16) < (setor_geral as i16) { // a extra mais atrás',
     'if (setor_extra as i16) > (setor_geral as i16) { // mutacao: trocado'),
]
