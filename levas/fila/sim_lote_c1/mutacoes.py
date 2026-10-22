# Mutacoes do rebaixamento da liga, da data da rodada da liga e da importancia do jogo. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('lc_rebaixamento', "    (*((liga + 0xc1) as *const u8) != 0) as u32 // tem rebaixamento", "    (*((liga + 0xc1) as *const u8) == 0) as u32 // tem rebaixamento"),
    ('lc_ano_relativo', "d[1].wrapping_sub(base as u16)); // ano relativo", "d[1].wrapping_sub(base as u16).wrapping_add(1)); // ano relativo"),
    ('lc_importancia', "    imp = imp.wrapping_add(10); // importância base", "    imp = imp.wrapping_add(11); // importância base"),
]
