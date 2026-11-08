# Mutacoes da leva167 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva167.py --carreira 2002-06-01
MUTACOES = [
    ('l_campo', "        if core::ptr::read_unaligned((e + 6) as *const i32) == *p {", "        if core::ptr::read_unaligned((e + 2) as *const i32) == *p {"),
    ('l_conta', "    while i < *(list as *const i16) as i32 {", "    while i < *(list as *const i16) as i32 - 1 {"),
    ('f_passo', "    (*(t as *const usize)).wrapping_add(k as usize * 0x50)", "    (*(t as *const usize)).wrapping_add(k as usize * 0x4c)"),
    ('d_jovem', "    if age < 0x13 && core::ptr::read_unaligned(e) < 0 {", "    if age < 0x12 && core::ptr::read_unaligned(e) < 0 {"),
    ('d_treze', "        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xd));", "        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xc));"),
    ('d_sorteio', "        let r = exe::utils_random(0xd) as i16;", "        let r = exe::utils_random(0xc) as i16;"),
    ('d_goleiro', "    if sheet != 0 && *((sheet + player_sheet::GOALKEEPER) as *const u8) == 0x14 && core::ptr::read_unaligned(e) >= 0x41a {", "    if sheet != 0 && *((sheet + player_sheet::GOALKEEPER) as *const u8) == 0x13 && core::ptr::read_unaligned(e) >= 0x41a {"),
]
