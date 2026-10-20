# Mutacoes do confronto direto e dos termos oferecidos. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_saldo', "            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),", "            1 => (w(a, 0x15), w(b, 0x15)),"),
    ('c_empate', "        somar(0xc, *((liga + 0xc3) as *const i8) as i16);", "        somar(0xc, *((liga + 0xc3) as *const i8) as i16 + 1);"),
    ('t_estado', "        *estado = (*estado == 6) as u8 + 7;", "        *estado = (*estado == 6) as u8 + 6;"),
]
