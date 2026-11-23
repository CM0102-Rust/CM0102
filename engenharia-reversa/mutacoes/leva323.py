# Mutações da 323a leva (itens mwb_22, mwb_23, mwb_24, mwb_25). Uso: python mutacoes.py mutacoes\leva323.py --carreira 2002-06-01
MUTACOES = [
    ('mwb22_sem_ordem_de_reputacao', 'exe::crt_qsort(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), n, 4, 0x4b07d0usize); // mwb22: the entrants by reputation', '// mutation: the entrants in table order', 'mwb_22 welsh league cup entrants'),
    ('mwb22_faixas_de_3', 'italian_cup_predicted_positions(this, 0xad56f4, (0x3b5, 0x3bd), 5, 6, ', 'italian_cup_predicted_positions(this, 0xad56f4, (0x3b5, 0x3bd), 5, 3, ', 'mwb_22 welsh league cup predicted'),
    ('mwb23_liga_7_dias_antes', '[(0x17, 7, 2, 2), (0x1e, 7, 2, 2), (6, 8, 2, 2)', '[(0x10, 7, 2, 2), (0x1e, 7, 2, 2), (6, 8, 2, 2)', 'mwb_23 welsh league cup groups'),
    ('mwb23_premier_7_dias_depois', '[(5, 8, 0, 1, 2), (3, 9, 0, 1, 2)', '[(0xc, 8, 0, 1, 2), (3, 9, 0, 1, 2)', 'mwb_23 welsh premier cup groups'),
    ('mwb24_ano_seguinte', "this.set_year(year as u16 as i16); // mwb24: the cup's year", 'this.set_year((year as u16 as i16).wrapping_add(1)); // mutation: next year', 'mwb_24 welsh league cup new'),
    ('mwb25_corte_trocado', '{ 8i8 } else { 7 }; // mwb25: the cut-off place', '{ 7i8 } else { 8 }; // mutation: the cut-off swapped', 'mwb_25 welsh premier cup entrants'),
    ('mwb25_faixas_de_4', 'italian_cup_predicted_positions(this, 0xad572c, (0x3db, 0x3e3), 1, 2, ', 'italian_cup_predicted_positions(this, 0xad572c, (0x3db, 0x3e3), 1, 4, ', 'mwb_25 welsh premier cup predicted'),
]
