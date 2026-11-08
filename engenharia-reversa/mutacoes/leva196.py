# Mutacoes do confronto direto e dos termos oferecidos. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_saldo', "            1 => (w(a, league_table_row::GOALS_FOR) - w(a, league_table_row::GOALS_AGAINST), w(b, league_table_row::GOALS_FOR) - w(b, league_table_row::GOALS_AGAINST)),", "            1 => (w(a, league_table_row::GOALS_FOR), w(b, league_table_row::GOALS_FOR)),"),
    ('c_empate', "        add(0xc, *((league + league::DRAW_POINTS) as *const i8) as i16);", "        add(0xc, *((league + league::DRAW_POINTS) as *const i8) as i16 + 1);"),
    ('t_estado', "        *state = (*state == 6) as u8 + 7;", "        *state = (*state == 6) as u8 + 6;"),
]
