# Mutacoes (cqp_4), na carreira de 8 ligas: a 1a rodada levada a outro dia, o Natal da terceira deslocado um dia (outro caso) e a ida das semifinais da segunda uma semana antes (maio de 2002).
MUTACOES = [
    ('cqp4_segunda_primeira_rodada_em_outro_dia',
     'const CQP4_SEGUNDA_COMECO: [Cqp4Rodada; 23] = [\n    (12, 7, 0, 5,',
     'const CQP4_SEGUNDA_COMECO: [Cqp4Rodada; 23] = [\n    (12, 7, 0, 2,',
     'cqp_4 calendario da segunda inglesa'),
    ('cqp4_terceira_natal_deslocado',
     'const CQP4_TERCEIRA_NATAL: Cqp4Natal = Cqp4Natal { comeco: &CQP4_SEGUNDA_COMECO, referencia: 22, ajuste: 13,',
     'const CQP4_TERCEIRA_NATAL: Cqp4Natal = Cqp4Natal { comeco: &CQP4_SEGUNDA_COMECO, referencia: 22, ajuste: 12,',
     'cqp_4 calendario da terceira inglesa'),
    ('cqp4_semifinais_da_segunda_uma_semana_antes',
     'const CQP4_SEGUNDA_PLAYOFF_0: Cql10R = cql10r((7, 4, 1, 0), (0xc,',
     'const CQP4_SEGUNDA_PLAYOFF_0: Cql10R = cql10r((7, 4, 1, 0), (0x5,',
     'cqp_4 play-offs da segunda inglesa'),
]
