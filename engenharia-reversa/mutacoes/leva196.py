# Mutacoes do confronto direto e dos termos oferecidos. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_saldo', "            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),", "            1 => (w(a, 0x15), w(b, 0x15)),"),
    ('c_empate', "        add(0xc, *((league + 0xc3) as *const i8) as i16);", "        add(0xc, *((league + 0xc3) as *const i8) as i16 + 1);"),
    ('t_estado', "        *state = (*state == 6) as u8 + 7;", "        *state = (*state == 6) as u8 + 6;"),
]
