# Mutacoes (cx07_6), carreira de 8 ligas ate 2004-11-01 (as quartas do futebol olimpico em 23 de setembro de 2004).
# - as quartas com o primeiro do grupo A contra o primeiro do B (outros jogos; acusada pela carreira: os jogadores
#   sub-23 das selecoes jogam outras partidas, com cansaco e lesoes nas ligas).
# A virada (start_new_season) fica sem mutacao propria: o que ela monta so e usado na edicao de 2008, fora da janela; o
# molde (nations_tournament_new_season do cx05_5) e provado pelas viradas da Copa Africana e da Asia.
# O advance_stage fica sem mutacao propria: mudar a fase do mata-mata deixa a fase 3 nula e o dia seguinte a le (trava).
MUTACOES = [
    ('cx07_6_quartas_a1_contra_b1',
     '    let order = [(0, 0), (1, 1), (2, 0), (3, 1), (0, 1), (1, 0), (2, 1), (3, 0)]; // cx07_6: (group, row) of each quarter-finalist',
     '    let order = [(0, 0), (1, 0), (2, 0), (3, 1), (0, 1), (1, 1), (2, 1), (3, 0)]; // mutation'),
]
