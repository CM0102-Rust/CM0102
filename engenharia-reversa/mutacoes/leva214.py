# Mutações da 214a leva (itens sim_lote_b1, sim_lote_b2, sim_lote_b3). Uso: python mutacoes.py mutacoes\leva214.py --carreira 2002-06-01
MUTACOES = [
    ('lt_pontos', '        return pb - pa; // pontos', '        return pa - pb; // pontos'),
    ('lt_grade', '                    *c = k; // vaga preenchida', '                    *c = k + 1; // vaga preenchida'),
    ('lt_ultimo_dia', '        return 0xff; // outside the last day', '        return 0xfe; // outside the last day'),
    ('cu_clube', '    d(0x1c, ponteiro(0xc, G_CLUB_TABLE, 0x245)); // home club', '    d(0x1c, ponteiro(0x10, G_CLUB_TABLE, 0x245)); // home club'),
    ('cu_sem_destaque', '    4 // sem destaque', '    3 // sem destaque'),
    ('cu_semana', '    *((e + 0x27) as *mut u8) = week as u8; // day of the week', '    *((e + 0x27) as *mut u8) = (week as u8).wrapping_add(1); // day of the week'),
    ('fm_espelho', '*q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho', '*q = 0xa0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho'),
    ('rg_mesma_nacao', '        s = s.wrapping_add(0x3e8); // same nation', '        s = s.wrapping_add(0x7d0); // same nation'),
    ('hm_outra_partida', '    if same { 1 } else { 0 } // outra partida', '    if same { 1 } else { 1 } // outra partida'),
]
