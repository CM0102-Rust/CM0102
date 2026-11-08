# Mutacoes das estatisticas da partida. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('j_campo', "    let deslocamento = if field == 1 { 0x20 } else { 0x22 };", "    let deslocamento = if field == 1 { 0x22 } else { 0x20 };"),
    ('e_data', "    (clubs && w(8) == data_a && w(0xa) == data_b) as i32", "    (clubs && w(8) == data_a) as i32"),
    ('b_tipo', "        return (*((game + 0x3a) as *const u8) == 1) as u8 + 2;", "        return (*((game + 0x3a) as *const u8) == 1) as u8 + 1;"),
]
