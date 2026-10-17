# Mutacoes da leva156 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva156.py --carreira 2002-06-01
MUTACOES = [
    ('t_numero', "        formatar(dest(out), 0x98806c, if t(pos + 2) == b'1' { numero.wrapping_add(1) } else { numero });", "        formatar(dest(out), 0x98806c, numero.wrapping_add(2));"),
    ('t_variante', "                    if resta <= 0 {", "                    if resta <= 1 {"),
    ('c_contagem', "                *((u(0xb67a50, 0) + 0x25) as *mut i32) += 1;", "                *((u(0xb67a50, 0) + 0x25) as *mut i32) += 2;"),
]
