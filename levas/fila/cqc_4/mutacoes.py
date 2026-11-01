# Mutacoes (cqc_4). O calendario roda a cada jogo marcado (7.650 entradas e 285 buscas de nova data ate 2002-06-01) e a
# data que sai e a que a carreira joga: a ordem dos dias da semana na busca com 1 antes de 2, o passo da busca de 3
# dias no lugar de 4 e o conflito do mesmo dia (gravidade 0x14) sem a resolucao (0x5ae380). Contadores colados no
# efeito, depois das condicoes do trecho.
MUTACOES = [
    ('cqc4_dias_outra_ordem',
     'let dias: [i16; 9] = [2, 1, -1, 5, 6, -1, 0, 3, 4]; // cqc4: a ordem dos dias da semana',
     'let dias: [i16; 9] = [1, 2, -1, 5, 6, -1, 0, 3, 4]; // mutacao: 1 antes de 2',
     'cqc_4 nova data'),
    ('cqc4_passo_de_3_dias',
     'let r = cqc4_op(0x54c5b0, pk, 4); // cqc4: o passo da busca',
     'let r = cqc4_op(0x54c5b0, pk, 3); // mutacao: de 3 em 3 dias',
     'cqc_4 nova data'),
    ('cqc4_mesmo_dia_sem_resolver',
     'mesmo_dia(this, jogo, l); // cqc4: o conflito do mesmo dia',
     'let _ = mesmo_dia; // mutacao: sem resolver',
     'cqc_4 mesmo dia'),
]
