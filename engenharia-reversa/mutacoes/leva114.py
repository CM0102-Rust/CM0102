# Mutacoes da leva114 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva114.py --jogo 2001-08-20
MUTACOES = [
    ('s_minuto', "        let min = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;", "        let min = (*((m + 0x8ed0) as *const i16) as i32 / 10) as i8;"),
    ('s_teto', "            0x3de => Some(0x5a),", "            0x3de => Some(0x59),"),
    ('s_19', "    *b(0x19) = -3;", "    *b(0x19) = -2;"),
    ('s_goleiro', "        chamar(0x6e1090, comissao, [1, 0, 0, 0, 0]);", "        chamar(0x6e1090, comissao, [0, 0, 0, 0, 0]);"),
    ('s_zera', "        *((m + if byte(j, 0x27) == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;", "        *((m + if byte(j, 0x27) != 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;"),
    ('s_guardar', "        *((m as isize + l * 2 + 0x8ecc) as *mut i16) = *((m + 0x8ed0) as *const i16);", "        *((m as isize + l * 2 + 0x8ecc) as *mut i16) = *((m + 0x8ed4) as *const i16);"),
    ('s_23', "        *b(0x23) = (*b(0x23)).wrapping_add(1);", "        *b(0x23) = (*b(0x23)).wrapping_add(2);"),
]
