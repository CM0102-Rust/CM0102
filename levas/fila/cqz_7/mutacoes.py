# Mutacoes (cqz_7): a primeira rodada do grupo da fase -1 sete dias depois (a tabela montada nas regras, na criacao, fica
# em +0xba: muda o banco da memoria) na Eurocopa e nas eliminatorias da Asia e da Oceania.
MUTACOES = [
    ('cqz7_euro_grupo_7_dias',
     'const EURO_CHAMP_OPS_2: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 9, 5,',
     'const EURO_CHAMP_OPS_2: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 0x10, 5,',
     'cqz_7 euro round table'),
    ('cqz7_asia_grupo_7_dias',
     'const WC_ASIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 0x1e, 3,',
     'const WC_ASIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 0x17, 3,',
     'cqz_7 asian qualifying round table'),
    ('cqz7_oceania_grupo_7_dias',
     'const WC_OCEANIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [\n    Cqp12D(0, 7, 3,',
     'const WC_OCEANIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [\n    Cqp12D(0, 0xe, 3,',
     'cqz_7 oceania qualifying round table'),
]
