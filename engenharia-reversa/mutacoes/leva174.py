# Mutacoes da leva167 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva167.py --carreira 2002-06-01
MUTACOES = [
    ('l_campo', "        if read_i32(e + 6) == *p {", "        if read_i32(e + 2) == *p {"),
    ('l_conta', "    let mut i = 0i32;\n    while i < read_i16(list.addr()) as i32 {", "    let mut i = 0i32;\n    while i < read_i16(list.addr()) as i32 - 1 {"),
    ('f_passo', "    read_usize(t).wrapping_add(k as usize * 0x50)", "    read_usize(t).wrapping_add(k as usize * 0x4c)"),
    ('d_jovem', "    if age < 0x13 && core::ptr::read_unaligned(e) < 0 {", "    if age < 0x12 && core::ptr::read_unaligned(e) < 0 {"),
    ('d_treze', "        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xd));", "        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xc));"),
    ('d_sorteio', "        let r = exe::utils_random(0xd) as i16;", "        let r = exe::utils_random(0xc) as i16;"),
    ('d_goleiro', "    if !sheet.is_null() && (sheet.goalkeeper() as u8) == 0x14 && core::ptr::read_unaligned(e) >= 0x41a {", "    if !sheet.is_null() && (sheet.goalkeeper() as u8) == 0x13 && core::ptr::read_unaligned(e) >= 0x41a {"),
]
