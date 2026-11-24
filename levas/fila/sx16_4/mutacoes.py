# Mutacoes (sx16_4, telas; provadas pelo comparador numa sessao do Lucas, na tela Official Stats).
# - os jogos em ordem decrescente viram crescentes: compare_official_stats_compare_games_desc acusa ao abrir a tela;
# - o segundo clique nao inverte (a coluna fica sempre na primeira ordem): o comparador do clique acusa.
MUTACOES = [
    ('sx16_4_jogos_invertidos',
     '    official_stats_by_value(a, b, |r| read_i16(r + official_stats::MATCHES) as i32, false)',
     '    official_stats_by_value(a, b, |r| read_i16(r + official_stats::MATCHES) as i32, true) // mutation'),
    ('sx16_4_sem_inverter',
     '            let again = column == number && reversed == 0;',
     '            let again = column == number && reversed == 2; // mutation'),
]
