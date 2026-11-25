# Mutacoes (rx11_5), ENG ate 2005-12-31 (a campanha de 2006): quem joga em cada grupo muda os jogos e o sorteio de cada
# partida, e a carreira acusa (as rotinas so rodam na original no comparador):
# - o sobrevivente da repescagem de cada grupo trocado pelo do grupo seguinte;
# - o grupo final com o primeiro e o terceiro de cada grupo semifinal.
MUTACOES = [
    ('rx11_5_repescagem_trocada',
     'write_usize(teams + 8, play_off[g].unwrap_or(0)); // rx11_5: one play-off survivor per group',
     'write_usize(teams + 8, play_off[(g + 1) % 3].unwrap_or(0)); // mutation: the next group\'s survivor',
     'rx11_5 concacaf semi-final groups'),
    ('rx11_5_final_com_o_terceiro',
     'write_usize(teams + (g as usize * 2 + row) * 4, read_usize(exe::league_get_table_row(group, row as i32))); // rx11_5: the top two of each group',
     'write_usize(teams + (g as usize * 2 + row) * 4, read_usize(exe::league_get_table_row(group, (row * 2) as i32))); // mutation: first and third',
     'rx11_5 concacaf final group'),
]
