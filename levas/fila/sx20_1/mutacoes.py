# Mutacoes (sx20_1). Os jogos simulados rodam na carreira ENG (todo dia, os grupos do dia); as estatisticas, lesoes
# e contratos dos jogadores de fora entram no banco (impressao) e nas compras da IA:
# - os grupos jogam no dia seguinte ao certo (outros jogos, outros sorteios);
# - os lesionados contam como livres (muda a ordem e os pares);
# - as estatisticas dos jogadores nao recebem o jogo (pula o PlayerStats::record_match).
MUTACOES = [
    ('sx20_1_outro_dia',
     'if today % 7 == g % 7 { // sx20_1: each group plays one day in seven',
     'if (today + 1) % 7 == g % 7 { // mutation: the next day',
     'sx20_1 simulated match'),
    ('sx20_1_lesionados_livres',
     '== 2 { // sx20_1: out injured',
     '== 0x7f { // mutation: the injured count as free',
     'sx20_1 simulated match'),
    ('sx20_1_sem_estatisticas',
     'for (owner, routine) in SIMULATED_MATCH_RECORDERS { // sx20_1: the routines that take the match',
     'for (owner, routine) in SIMULATED_MATCH_RECORDERS.into_iter().skip(1) { // mutation: no player statistics',
     'sx20_1 simulated match'),
]
