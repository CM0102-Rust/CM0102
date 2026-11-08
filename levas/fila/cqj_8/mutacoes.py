# Mutacoes (cqj_8): as regras da NSL e da MLS (montagem da carreira; a MLS joga ate outubro de 2001 e a NSL de outubro
# de 2001 a maio de 2002, antes de 2002-06-01) e o historico dos estaduais (fim de cada estadual, no primeiro semestre
# de 2002: 9 chamadas na carreira de 8 ligas).
# - NSL: a vitoria vale 2 (a tabela e os seis da fase final mudam).
# - MLS: o empate vale 0.
# - Estaduais: o historico do primeiro estagio (campeao, vice...) nao e gravado (os registros do historico e a Copa dos
#   Campeoes, que procura os campeoes, mudam).
MUTACOES = [
    ('cqj8_vitoria_nsl', 'cqj8_b(o, 0xc2, 3); // cqj8: os pontos por vitória da NSL',
     'cqj8_b(o, 0xc2, 2); // mutacao: os pontos por vitória da NSL', 'cqj_8 regras da nsl'),
    ('cqj8_empate_mls', 'cqj8_b(o, 0xc3, 1); // cqj8: os pontos por empate da MLS',
     'cqj8_b(o, 0xc3, 0); // mutacao: os pontos por empate da MLS', 'cqj_8 regras da mls'),
    ('cqj8_sem_historico_estadual', 'record_first_stage_honours(this); // cqj8: o histórico do estadual',
     '// mutacao: sem o histórico do estadual', 'cqj_8 historico do estadual'),
]
