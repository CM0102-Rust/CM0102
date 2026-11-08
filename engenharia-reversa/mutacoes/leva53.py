# Mutacoes da leva53 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva53.py
MUTACOES = [
    ('i_10', "mark(table, pos) & 0x10 == 0 ||", "mark(table, pos) & 0x20 == 0 ||"),
    ('i_4', "mark(table, byte(p, 0x19)) & 4 == 0 {", "mark(table, byte(p, 0x19)) & 1 == 0 {"),
    ('i_outro', "            let other = (l == 0) as isize;\n            let q =", "            let other = l as isize;\n            let q ="),
    ('i_ficha', "marcar(table, byte(p, 0x19), *((q + match_player::STAFF) as *const usize) as *const u32);", "marcar(table, byte(p, 0x19), *((q + match_player::PLAYER) as *const usize) as *const u32);"),
    ('i_lado', "        let table = this + engine::TACTICS + 0x18e3 * l as usize;", "        let table = this + engine::TACTICS;"),
]
