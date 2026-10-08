# Mutacoes da leva53 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva53.py
MUTACOES = [
    ('i_10', "marca(tabela, pos) & 0x10 == 0 ||", "marca(tabela, pos) & 0x20 == 0 ||"),
    ('i_4', "marca(tabela, byte(p, 0x19)) & 4 == 0 {", "marca(tabela, byte(p, 0x19)) & 1 == 0 {"),
    ('i_outro', "            let outro = (l == 0) as isize;\n            let q =", "            let outro = l as isize;\n            let q ="),
    ('i_ficha', "marcar(tabela, byte(p, 0x19), *((q + 0x69) as *const usize) as *const u32);", "marcar(tabela, byte(p, 0x19), *((q + 0x6d) as *const usize) as *const u32);"),
    ('i_lado', "        let tabela = this + 0x91e2 + 0x18e3 * l as usize;", "        let tabela = this + 0x91e2;"),
]
