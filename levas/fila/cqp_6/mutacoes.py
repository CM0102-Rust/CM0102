# Mutacoes (cqp_6), na carreira de 8 ligas: a 1a rodada da tabela do ano impar (2001) levada a outro dia e o Natal dela deslocado um dia (outro caso).
MUTACOES = [
    ('cqp6_primera_impar_primeira_rodada_em_outro_dia',
     'const CQP6_PRIMERA_IMPAR_COMECO: [Cqp6Rodada; 16] = [\n    (22, 7, 0, 6,',
     'const CQP6_PRIMERA_IMPAR_COMECO: [Cqp6Rodada; 16] = [\n    (22, 7, 0, 2,',
     'cqp_6 calendario da primera'),
    ('cqp6_primera_impar_natal_deslocado',
     'const CQP6_PRIMERA_IMPAR_NATAL: Cqp6Natal = Cqp6Natal { comeco: &CQP6_PRIMERA_IMPAR_COMECO, referencia: 15, ajuste: 16,',
     'const CQP6_PRIMERA_IMPAR_NATAL: Cqp6Natal = Cqp6Natal { comeco: &CQP6_PRIMERA_IMPAR_COMECO, referencia: 15, ajuste: 15,',
     'cqp_6 calendario da primera'),
]
