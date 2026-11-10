# Mutacoes da leva68 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva68.py
MUTACOES = [
    ('v_vazia', "        if i != -1 && *(persons.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) == read_i32(linha + tactics_row::CAPTAIN) {", "        if i != -1 && *(persons.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) != read_i32(linha + tactics_row::CAPTAIN) {"),
    ('v_menos', "    -((minus_one != 0) as i8)", "    -((minus_one == 0) as i8)"),
    ('v_20', "    for vaga in 0..20usize {", "    for vaga in 0..19usize {"),
    ('m_52', "    write_i32(linha + tactics_row::CAPTAIN, i);\n    1", "    write_i32(linha + tactics_row::CAPTAIN, i + 1);\n    1"),
    ('m_vazia', "    let i = read_i32(linha + tactics_row::SELECTION + 4 * slot as usize);\n    if i == -1 {", "    let i = read_i32(linha + tactics_row::SELECTION + 4 * slot as usize);\n    if i == -2 {"),
    ('p_56', "    let i = read_i32(linha + tactics_row::PLAYMAKER + 4 * which as usize);", "    let i = read_i32(linha + tactics_row::PLAYMAKER + 4 * (1 - which) as usize);"),
    ('p_neg', "    let i = read_i32(linha + tactics_row::PLAYMAKER + 4 * which as usize);\n    if i < 0 {", "    let i = read_i32(linha + tactics_row::PLAYMAKER + 4 * which as usize);\n    if i < -1 {"),
]
