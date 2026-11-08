# Mutações da 212a leva (itens sim_match_stats_2). Uso: python mutacoes.py mutacoes\leva212.py --carreira 2002-06-01
MUTACOES = [
    ('r_minuto', '    pw(reg + match_record::MINUTE, (w(m + engine::CLOCK) as i16 as i32 / 11) as u16);', '    pw(reg + match_record::MINUTE, (w(m + engine::CLOCK) as i16 as i32 / 10) as u16);'),
    ('o_minuto', '    let r = *a.add(3) as i32 - *b.add(3) as i32;', '    let r = *b.add(3) as i32 - *a.add(3) as i32;'),
    ('i_ultima', '    *((last - 2) as *mut u8) = 0;', '    *((last - 2) as *mut u8) = 1;'),
]
