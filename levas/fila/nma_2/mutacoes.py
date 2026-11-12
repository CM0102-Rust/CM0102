# Mutacoes (nma_2). A passada das temporadas roda todo dia; na carreira ENG ate 2002-06-01 o dia anterior ao dia de
# atualizacao de uma entrada fecha a temporada da nacao 40 vezes (0x447880, 0x7c1350, 0x59cdd0 e 0x86d6e0 com 40
# chamadas na cobertura) e uma competicao recomeca (0x556210 com 1 chamada):
# - sem o fechamento das financas da nacao (orcamentos e caixa dos clubes mudam as compras da IA);
# - sem o fechamento das estatisticas de temporada dos jogadores da nacao;
# - a competicao nao recomeca a temporada (o START_NEW_SEASON da competicao completa).
MUTACOES = [
    ('nma2_sem_financas',
     'by_entry(season_pass::FN_FINANCE_END_SEASON, read_usize(G_FINANCE_TABLE)); // nma2: finances',
     '(); // mutation: the finances do not close the season',
     'nma_2 day before'),
    ('nma2_sem_estatisticas',
     'by_entry(season_pass::FN_PLAYER_STATS_END_SEASON, read_usize(G_PLAYER_STATS)); // nma2: player stats',
     '(); // mutation: the player stats do not close the season',
     'nma_2 day before'),
    ('nma2_sem_reinicio',
     "start(comp); // nma2: the competition's new season",
     'let _ = start; // mutation: the competition does not restart',
     'nma_2 comp restart'),
]
