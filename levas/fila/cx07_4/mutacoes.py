# Mutacoes (cx07_4), carreira de 8 ligas ate 2002-12-31 (o mata-mata da Copa da Oceania em 17 e 19 de novembro de 2002).
# - as semifinais com os dois primeiros e os dois segundos frente a frente (outros jogos; acusada pela carreira: os
#   jogadores da Australia, da liga AUS, jogam outras partidas).
# A virada (start_new_season) fica sem mutacao propria: o que ela monta so e usado na edicao de 2004, fora da janela; o
# molde (nations_tournament_new_season do cx05_5) e provado pelas viradas da Copa Africana e da Asia.
# O advance_stage fica sem mutacao propria: mudar a fase do mata-mata deixa a fase 1 nula e o dia seguinte a le (trava).
MUTACOES = [
    ('cx07_4_semifinais_primeiros_contra_primeiros',
     '    let pairs = [(this, 0), (group_b, 1), (this, 1), (group_b, 0)]; // cx07_4: the semi-final pairs',
     '    let pairs = [(this, 0), (group_b, 0), (this, 1), (group_b, 1)]; // mutation'),
]
