# Mutacoes (cx09_9) pela carreira com a Belgica ate 2002-08-01:
# - o campeao do periodo nao e marcado na tabela do grupo: a repescagem de acesso (que completa os marcados ate tres por
#   grupo) muda de clubes e quem sobe muda;
# - o progresso da liga multiplica por 2 (e nao por 3): o comparador do progresso acusa (as estatisticas da competicao
#   mudam).
MUTACOES = [
    ('cx09_9_campeao_sem_marca',
     '                row.set_status(1);\n                news(this, row.club() as usize, period);',
     '                row.set_status(0xff); // mutation\n                news(this, row.club() as usize, period);'),
    ('cx09_9_progresso_vezes_dois',
     '        (c, b, w) = (c.wrapping_mul(3).wrapping_sub(2), b.wrapping_mul(3).wrapping_sub(2), w.wrapping_mul(3).wrapping_sub(2));',
     '        (c, b, w) = (c.wrapping_mul(2).wrapping_sub(2), b.wrapping_mul(3).wrapping_sub(2), w.wrapping_mul(3).wrapping_sub(2)); // mutation'),
]
