# Mutacoes da leva92 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva92.py --jogo 2001-08-20
MUTACOES = [
    ('a_sobe', "    let sobe = read_i8(this + match_player::SIDE) == 1;\n    if sobe {\n        if y < 11 {", "    let sobe = read_i8(this + match_player::SIDE) != 1;\n    if sobe {\n        if y < 11 {"),
    ('a_11', "    if sobe {\n        if y < 11 {", "    if sobe {\n        if y < 10 {"),
    ('a_0', "    } else if y > 0 {\n        y -= 1;", "    } else if y > 1 {\n        y -= 1;"),
    ('a_linha', "read_i8(engine + engine::OFFSIDE_LINE_SIDE1).wrapping_add(1)", "read_i8(engine + engine::OFFSIDE_LINE_SIDE1)"),
    ('a_direto', "    if direct != 0 {\n        let row", "    if direct == 0 {\n        let row"),
    ('a_menos1', "    call(FN_MATCH_PLAYER_MOVE_TO, [x as u8 as u32, y as u8 as u32, u32::MAX, 0, 0])", "    call(FN_MATCH_PLAYER_MOVE_TO, [x as u8 as u32, y as u8 as u32, 0, 0, 0])"),
]
