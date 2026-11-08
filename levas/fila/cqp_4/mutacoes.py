# Mutacoes (cqp_4), na carreira de 8 ligas: a 1a rodada levada a outro dia, o Natal da terceira deslocado um dia (outro caso) e a ida das semifinais da segunda uma semana antes (maio de 2002).
MUTACOES = [
    ('cqp4_segunda_primeira_rodada_em_outro_dia',
     'const SECOND_DIVISION_START: [EngSecondScheduleRound; 23] = [\n    (12, 7, 0, 5,',
     'const SECOND_DIVISION_START: [EngSecondScheduleRound; 23] = [\n    (12, 7, 0, 2,',
     'cqp_4 english second division calendar'),
    ('cqp4_terceira_natal_deslocado',
     'const THIRD_DIVISION_CHRISTMAS_SCHEDULE: EngSecondChristmasSchedule = EngSecondChristmasSchedule { comeco: &SECOND_DIVISION_START, referencia: 22, ajuste: 13,',
     'const THIRD_DIVISION_CHRISTMAS_SCHEDULE: EngSecondChristmasSchedule = EngSecondChristmasSchedule { comeco: &SECOND_DIVISION_START, referencia: 22, ajuste: 12,',
     'cqp_4 english third division calendar'),
    ('cqp4_semifinais_da_segunda_uma_semana_antes',
     'const DIVISION_TWO_PLAYOFF_0: EngCcCupCupRound = cql10r((7, 4, 1, 0), (0xc,',
     'const DIVISION_TWO_PLAYOFF_0: EngCcCupCupRound = cql10r((7, 4, 1, 0), (0x5,',
     'cqp_4 english second division play-offs'),
]
