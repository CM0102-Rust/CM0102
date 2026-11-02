# Mutacoes (cqg_8), todas na 0x5b5d10 (a formacao lida do .pct: 45 na carreira automatica ate 2002-06-01, as das
# taticas da IA e as pre-definidas na criacao do jogo). As posicoes de cada vaga, refeitas pela casa guardada
# (0x5b3560), decidem a escalacao da IA (os jogadores por posicao), que entra nas partidas simuladas.
# - A casa da vaga gravada na fase 1 em vez da 0 (as casas da fase 1 mudam e as posicoes sao refeitas por elas).
# - A casa da vaga gravada na faixa 1 em vez da 0.
# - A casa lida da fase 1 e gravada na fase 0.
MUTACOES = [
    ('cqg8_casa_na_fase_1', 'refazer(this, vaga, 0, 0, c); // cqg8: as posições refeitas pela casa',
     'refazer(this, vaga, 1, 0, c); // mutacao: as posições refeitas pela casa', 'cqg_8 posicoes refeitas'),
    ('cqg8_casa_na_faixa_1', 'refazer(this, vaga, 0, 0, c); // cqg8: as posições refeitas pela casa',
     'refazer(this, vaga, 0, 1, c); // mutacao: as posições refeitas pela casa', 'cqg_8 posicoes refeitas'),
    ('cqg8_casa_lida_da_fase_1', 'guardada(this, vaga as i8, 0, 0, &mut c); // cqg8: a casa guardada da fase 0',
     'guardada(this, vaga as i8, 1, 0, &mut c); // mutacao: a casa guardada da fase 0', 'cqg_8 posicoes refeitas'),
]
