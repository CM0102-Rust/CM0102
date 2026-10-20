# Mutacoes do registro da partida, da ordem dos eventos e do idioma. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('r_minuto', "    pw(reg + 0x4d, (w(m + 0x8ed0) as i16 as i32 / 11) as u16);", "    pw(reg + 0x4d, (w(m + 0x8ed0) as i16 as i32 / 10) as u16);"),
    ('o_minuto', "    let r = *a.add(3) as i32 - *b.add(3) as i32;", "    let r = *b.add(3) as i32 - *a.add(3) as i32;"),
    ('i_ultima', "    *((ultima - 2) as *mut u8) = 0;", "    *((ultima - 2) as *mut u8) = 1;"),
]
