# Mutacoes do match_wrap_up do motor (2). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('z_lado', "    let f: unsafe extern \"thiscall\" fn(usize) -> u8 = core::mem::transmute(FN_FORMATION_HAS_NO_WIDE_DEFENDERS);\n    f(m + if side != 0 { 0xaac5 } else { 0x91e2 })", "    let f: unsafe extern \"thiscall\" fn(usize) -> u8 = core::mem::transmute(FN_FORMATION_HAS_NO_WIDE_DEFENDERS);\n    f(m + if side == 0 { 0xaac5 } else { 0x91e2 })"),
    ('a_motor', "    core::ptr::write_unaligned(a as *mut u32, 0);\n    core::ptr::write_unaligned((a + 4) as *mut u32, 0);", "    core::ptr::write_unaligned((a + 4) as *mut u32, 0);"),
    ('j_destrutor', "    core::ptr::write_unaligned((j + match_player::ENGINE) as *mut u32, 0);", "    core::ptr::write_unaligned((j + match_player::ENGINE) as *mut u32, 1);"),
]
