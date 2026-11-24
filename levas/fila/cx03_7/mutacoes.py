# Mutacoes (cx03_7), carreira com as ligas da Polonia/Russia ate 2002-08-01. Nenhuma abre caixa.
# - a Copa da Russia vira uma semana depois (8 de junho): os inscritos, o sorteio e os jogos da edicao nova mudam de
#   data (a carreira acusa; o comparador so roda a original).
MUTACOES = [
    ('cx03_7_virada_uma_semana_depois',
     '    update_cup_with_turnover_on(Cup(this), today, stage, 1, 5, "cx03_7 russian cup next edition")',
     '    update_cup_with_turnover_on(Cup(this), today, stage, 8, 5, "cx03_7 russian cup next edition") // mutation'),
]
