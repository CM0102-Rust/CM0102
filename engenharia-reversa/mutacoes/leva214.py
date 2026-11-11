# Mutações da 214a leva (itens sim_lote_b1, sim_lote_b2, sim_lote_b3). Uso: python mutacoes.py mutacoes\leva214.py --carreira 2002-06-01
MUTACOES = [
    ('lt_pontos', '        return pb - pa; // pontos', '        return pa - pb; // pontos'),
    ('lt_grade', '                    *c = k; // place filled', '                    *c = k + 1; // place filled'),
    ('lt_ultimo_dia', '        return 0xff; // outside the last day', '        return 0xfe; // outside the last day'),
    ('cu_clube', '    d(fixture::HOME_CLUB, pointer(fixture::HOME_CLUB_ID, G_CLUB_TABLE, 0x245)); // home club', '    d(0x1c, pointer(0x10, G_CLUB_TABLE, 0x245)); // home club'),
    ('cu_sem_destaque', '    4 // no highlight', '    3 // no highlight'),
    ('cu_semana', '    e.set_slot_weekday(week as u8); // day of the week', '    e.set_slot_weekday((week as u8).wrapping_add(1)); // day of the week'),
    ('fm_espelho', '*q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho', '*q = 0xa0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho'),
    ('rg_mesma_nacao', '        s = s.wrapping_add(0x3e8); // same nation', '        s = s.wrapping_add(0x7d0); // same nation'),
    ('hm_outra_partida', '    if same { 1 } else { 0 } // another match', '    if same { 1 } else { 1 } // another match'),
]
