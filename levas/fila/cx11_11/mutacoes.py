# Mutacoes (cx11_11), com a Italia e a base NORMAL ate 2002-09-30:
# - o primeiro jogo da lista com o mando trocado: o jogo da 1a rodada (2 de setembro de 2001) e no outro estadio (a
#   renda, o publico e o fator casa mudam). Escapa se algum nome de 2001/02 faltar no banco da carreira (a lista cai,
#   como na C1 com a base minima, cqm_4/cqn_5); entao a prova diz isso.
MUTACOES = [
    ('cx11_11_mando_trocado',
     '(2, 8, 2001, 0x9ea744, 0x9ea750), // Cremonese × Poggibonsi',
     '(2, 8, 2001, 0x9ea750, 0x9ea744), // mutation: Poggibonsi × Cremonese',
     'cx11_11 serie c2 fixed calendar'),
]
