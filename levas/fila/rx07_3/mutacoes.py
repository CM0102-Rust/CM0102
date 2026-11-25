# Mutações (rx07_3), carreira ENG até 2002-06-01. A tabela das datas é a mesma que o comparador usa: a comparação
# acusa na criação do jogo, e as datas mudadas valem até o primeiro dia dos contratos (que recalcula as dez primeiras).
# - a última data da regra de transferências fica em hoje + 98 dias, não 91.
MUTACOES = [
    ('rx07_3_regra_98_dias',
     '    (contract_day_globals::RULE_PLUS_91_DAYS, 0x5b), // rx07_3: the last rule date',
     '    (contract_day_globals::RULE_PLUS_91_DAYS, 0x62), // rx07_3: the last rule date (mutation)',
     'rx07_3 contract dates'),
]
