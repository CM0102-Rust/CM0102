# Mutacoes da leva53 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva53.py
MUTACOES = [
    ('i_10', "exe::formation_get_marking_type(table, pos) & 0x10 == 0 ||", "exe::formation_get_marking_type(table, pos) & 0x20 == 0 ||"),
    ('i_4', "exe::formation_get_marking_type(table, byte(p, match_player::POSITION)) & 4 == 0 {", "exe::formation_get_marking_type(table, byte(p, match_player::POSITION)) & 1 == 0 {"),
    ('i_outro', "            let other = (l == 0) as isize;\n            let q =", "            let other = l as isize;\n            let q ="),
    ('i_ficha', "marcar(table, byte(p, match_player::POSITION), *((q + match_player::STAFF) as *const usize) as *const u32);", "marcar(table, byte(p, match_player::POSITION), *((q + match_player::PLAYER) as *const usize) as *const u32);"),
    ('i_lado', "        let table = this + engine::TACTICS + 0x18e3 * l as usize;", "        let table = this + engine::TACTICS;"),
]
