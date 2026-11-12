# Mutacoes (cqx_9), nas viradas da Supercopa da Asia e da Copa dos Campeoes da CONCACAF (uma chamada de cada ate
# 2002-06-01; a data nao se sabe sem rodar):
# - a Supercopa vira em qualquer mes (se a chamada nao cai em junho, a virada passa a acontecer: libera, arquiva,
#   sorteia a primeira rodada; sem contador, porque fora de junho o ramo da virada nao roda na base);
# - a Supercopa vira sem a primeira rodada (se a chamada cai em junho: um sorteio a menos);
# - a CONCACAF sem a arvore dos confrontos (os clubes da MLS nao jogam a edicao nova);
# - o ano das duas sobe 2 em vez de 1 (as datas da edicao nova saem da janela).
MUTACOES = [
    ('cqx9_supercopa_em_qualquer_mes',
     'if month != 5 { // cqx9: the Asian Super Cup turns over in June',
     'if false && month != 5 { // mutation: in any month'),
    ('cqx9_supercopa_sem_primeira_rodada',
     'exe::cup_set_up_round(cup, 0); // cqx9: the first round of the new Super Cup',
     '// mutation: no first round',
     'cqx_9 asian super cup turnover'),
    ('cqx9_concacaf_sem_arvore',
     'if cup_turnover_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 { // cqx9: the CONCACAF tie tree',
     'if false { // mutation: no tie tree',
     'cqx_9 concacaf champions cup turnover'),
    ('cqx9_ano_mais_dois',
     'cup.set_year(cup.year().wrapping_add(1)); // cqx9: the next season\'s cup',
     'cup.set_year(cup.year().wrapping_add(2)); // mutation: two years on',
     'cqx_9 concacaf champions cup turnover'),
]
