# Mutacoes (cqz_6): a primeira rodada do grupo da fase -1 (a tabela montada nas regras, na criacao, e guardada em +0xba:
# muda o banco da memoria) 7 dias depois na Copa da Asia e na Olimpiada; a Copa da Oceania de 2002 com as datas de junho
# (o sorteio de marco de 2002 monta os jogos com as outras datas).
MUTACOES = [
    ('cqz6_asia_grupo_7_dias',
     'const ASIA_NATIONS_OPS_2: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 0xd, 9,',
     'const ASIA_NATIONS_OPS_2: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 0x14, 9,',
     'cqz_6 asian cup round table'),
    ('cqz6_oceania_datas_de_junho',
     'if (this.year() as i32) % 4 == 2 { // cqz6: the Oceania Nations Cup of 2002, 2006...',
     'if (this.year() as i32) % 4 == 3 { // mutation: the June dates in 2002',
     'cqz_6 oceania nations cup round table'),
    ('cqz6_olimpiada_grupo_7_dias',
     'const OLYMPICS_OPS_2: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 0xd, 8,',
     'const OLYMPICS_OPS_2: [GoldcupScheduleOp; 12] = [\n    Cqp12D(0, 0x14, 8,',
     'cqz_6 olympics round table'),
]
