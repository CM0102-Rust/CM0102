# Mutacoes da leva131 (ver docs/ROTINAS.md). Sem comparacao lado a lado: so pela carreira.
# Uso: python mutacoes.py mutacoes\leva131.py --carreira 2002-06-01
MUTACOES = [
    ('j_ficha45', "*((f + 0x45) as *mut u8) = *((row - 4) as *const u8);", "*((f + 0x45) as *mut u8) = *((row - 3) as *const u8);"),
    ('j_fichab', "core::ptr::write_unaligned((f + 0xb) as *mut i16, w(row + 3));", "core::ptr::write_unaligned((f + 0xb) as *mut i16, w(row + 5));"),
    ('j_ficha9', "core::ptr::write_unaligned((g(p + staff::PLAYER) + 9) as *mut i16, w(row + 5));", "core::ptr::write_unaligned((g(p + staff::PLAYER) + 9) as *mut i16, w(row + 7));"),
    ('j_tec', "da = w(ts + 0x188).wrapping_sub(w(f + 0xa));", "da = w(ts + 0x18a).wrapping_sub(w(f + 0xa));", "players manager"),
    ('j_aux', "ftol(dv as f64 * 0.25)", "ftol(dv as f64 * 0.3)", "players manager"),
    ('j_limite', "w(f + o).clamp(1, 10000)", "w(f + o).clamp(1, 9000)", "players manager"),
    ('j_80', "club.set_reputation(w(ts + 0x18e));", "club.set_reputation(w(ts + 0x18c));"),
    ('j_n11_media', "&& average() + 0.2 > nota11", "&& average() + 0.3 > nota11", "players news 0x11"),
    ('j_n11_sorteio', "&& ((random(10) + 5) as f64) < nota1", "&& ((random(10) + 4) as f64) < nota1", "players news 0x11"),
    ('j_n11_nota', "d * 1.1 < w(row + 3) as f64", "d * 1.2 < w(row + 3) as f64", "players news 0x11"),
    ('j_n34', "if r as f64 * 0.1 + m + 0.5 < nota11 {", "if r as f64 * 0.1 + m + 0.4 < nota11 {", "players news 0x34"),
    ('j_n34_linha', "if *(row as *const u8) >= 9 && scoreboard(ts)", "if *(row as *const u8) >= 8 && scoreboard(ts)", "players news 0x34"),
]
