# Mutacoes (cx03_13), carreira de 8 ligas ate 2002-08-01. Nenhuma abre caixa.
# - as supercopas com a tabela de rodadas e o premio zerado: o comparador das rodadas acusa na criacao;
# - o premio das copas asiaticas 3.000.000: o comparador acusa na criacao;
# - a Supercopa da Asia vira em 25 de julho: a virada (que so age em junho) nao acontece; sem a edicao nova, o sorteio da
#   primeira rodada nao gasta o gerador (a carreira).
MUTACOES = [
    ('cx03_13_supercopas_sem_premio',
     '    build_cup_round_table_by_vtable(Cup(this));\n    1',
     '    build_cup_round_table_by_vtable(Cup(this));\n    Cup(this).set_prize_fund(0); // mutation\n    1'),
    ('cx03_13_premio_copas_asiaticas',
     '    set_up_prize_rounds(Cup(this), 4_000_000)',
     '    set_up_prize_rounds(Cup(this), 3_000_000) // mutation'),
    ('cx03_13_supercopa_da_asia_em_julho',
     'update_cup_with_turnover_on(Cup(this), today, stage, 0x19, 5, "cx03_13 asian super cup next edition")',
     'update_cup_with_turnover_on(Cup(this), today, stage, 0x19, 6, "cx03_13 asian super cup next edition") // mutation'),
]
