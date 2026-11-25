# Mutacoes (rx11_3), ENG ate 2005-12-31 (a campanha de 2006): as fases so rodam na original no comparador; quem joga
# com quem muda os jogos das eliminatorias e o sorteio de cada partida, e a carreira acusa:
# - os potes trocados por blocos (o grupo g fica com os inscritos g x tamanho .. g x tamanho + tamanho - 1);
# - os segundos dos grupos na repescagem em vez dos primeiros.
# A advance_stage fica sem mutacao propria (o molde do cx16_10): trocar a fase que cria os grupos deixa fases nulas que
# derrubam o jogo; o comparador confere que a fase anda.
MUTACOES = [
    ('rx11_3_potes_em_blocos',
     'write_usize(teams + k * 4, read_usize(list + (g + k * groups) * 4)); // rx11_3: the pots dealt across the groups',
     'write_usize(teams + k * 4, read_usize(list + (g * size + k) * 4)); // mutation: blocks instead of pots',
     'rx11_3 african groups'),
    ('rx11_3_segundos_na_repescagem',
     "write_usize(teams + k * 4, read_usize(group.table())); // rx11_3: each group's winner",
     'write_usize(teams + k * 4, read_usize(group.table() + league_table_row::SIZE)); // mutation: the runners-up',
     'rx11_3 african play-off'),
]
