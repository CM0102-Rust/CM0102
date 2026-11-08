# Mutacoes da leva164 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva164.py --carreira 2002-06-01
MUTACOES = [
    ('g_especial', "            2 => 4,\n            5 => 3,", "            2 => 4,\n            5 => 2,"),
    ('g_meio', "        return if p == n / 2 { n - 1 } else if p == n { n / 2 } else { p };", "        return if p == n / 2 { n - 2 } else if p == n { n / 2 } else { p };"),
    ('g_salto', "            if pos < n - 5 && sorteio(4) >= 2 {", "            if pos < n - 4 && sorteio(4) >= 2 {"),
    ('g_volta', "    let volta = |p: i32| if p < n { p } else { p - n };", "    let volta = |p: i32| if p <= n { p } else { p - n };"),
    ('r_b', "    *((e + 0xa) as *mut i8) = b;", "    *((e + 0xa) as *mut i8) = c;"),
    ('a_2001', "            let w = if year == 0x7d1 { 0 } else { v | 1 };", "            let w = if year == 0x7d2 { 0 } else { v | 1 };"),
    ('a_sobe', "                core::ptr::write_unaligned(e, year.wrapping_mul(2) | v & 1);", "                core::ptr::write_unaligned(e, year.wrapping_mul(2) | 1);"),
]
