# Mutacoes da leva98 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva98.py --jogo 2001-08-20
MUTACOES = [
    ('p_2b', "    j.set_busy((j.busy()).wrapping_add(1));\n    let dir = j.side();", "    j.set_busy((j.busy()).wrapping_add(2));\n    let dir = j.side();"),
    ('p_36', "    if kind != 0x36 {", "    if kind != 0x37 {"),
    ('p_dobro', "            let n = ftol(d2 as f64 * d1 as f64 + d2 as f64 * d1 as f64);", "            let n = ftol(d2 as f64 * d1 as f64);"),
    ('p_13', "                q.set_offsides((((q.offsides() as i8)).wrapping_add(1)) as u8);", "                q.set_offsides((((q.offsides() as i8)).wrapping_add(2)) as u8);"),
    ('p_posse', "            *other = (*other).wrapping_sub(10);", "            *other = (*other).wrapping_sub(9);"),
    ('p_80', "        let r = random(0x50);", "        let r = random(0x51);"),
    ('p_8eb2', "                *((m() + 0x8eb2) as *mut i8) = 7;", "                *((m() + 0x8eb2) as *mut i8) = 6;"),
    ('p_completo', "    if read_i8(mm + engine::RESTART_TYPE) != 0 || full == 0 {", "    if read_i8(mm + engine::RESTART_TYPE) != 0 || full == 1 {"),
]
