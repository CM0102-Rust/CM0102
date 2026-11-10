# Mutacoes do confronto direto e dos termos oferecidos. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_saldo', "            1 => ((a.goals_for() as i32) - (a.goals_against() as i32), (b.goals_for() as i32) - (b.goals_against() as i32)),", "            1 => ((a.goals_for() as i32), (b.goals_for() as i32)),"),
    ('c_empate', "        add(0xc, (league.draw_points() as i8) as i16);", "        add(0xc, (league.draw_points() as i8) as i16 + 1);"),
    ('t_estado', "        *state = (*state == 6) as u8 + 7;", "        *state = (*state == 6) as u8 + 6;"),
]
