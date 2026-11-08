# Mutacoes (cqq_5), na carreira de 8 ligas: a 1a rodada da tabela do ano impar (2001) levada a outro dia e o Natal dela deslocado um dia (outro caso: em 2001 a rodada 19 cai em 23/dez, caso 6; com o ajuste 18, caso 5).
MUTACOES = [
    ('cqq5_baixo_impar_primeira_rodada_em_outro_dia',
     'const LOWER_ODD_EARLY_ROUNDS: [SpaLowerLeagueRound; 20] = [\n    (16, 7, 0, 6,',
     'const LOWER_ODD_EARLY_ROUNDS: [SpaLowerLeagueRound; 20] = [\n    (16, 7, 0, 2,',
     'cqq_5 calendario da divisao de baixo espanhola'),
    ('cqq5_baixo_impar_natal_deslocado',
     'const LOWER_ODD_YEAR_CHRISTMAS: SpaLowerChristmasSchedule = SpaLowerChristmasSchedule { comeco: &LOWER_ODD_EARLY_ROUNDS, referencia: 19, ajuste: 17,',
     'const LOWER_ODD_YEAR_CHRISTMAS: SpaLowerChristmasSchedule = SpaLowerChristmasSchedule { comeco: &LOWER_ODD_EARLY_ROUNDS, referencia: 19, ajuste: 18,',
     'cqq_5 calendario da divisao de baixo espanhola'),
]
