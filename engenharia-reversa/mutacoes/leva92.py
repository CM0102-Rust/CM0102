# Mutacoes da leva92 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva92.py --jogo 2001-08-20
MUTACOES = [
    ('a_sobe', "    let sobe = byte(this, 0x27) == 1;\n    if sobe {\n        if y < 11 {", "    let sobe = byte(this, 0x27) != 1;\n    if sobe {\n        if y < 11 {"),
    ('a_11', "    if sobe {\n        if y < 11 {", "    if sobe {\n        if y < 10 {"),
    ('a_0', "    } else if y > 0 {\n        y -= 1;", "    } else if y > 1 {\n        y -= 1;"),
    ('a_linha', "byte(motor, 0x8eb0).wrapping_add(1)", "byte(motor, 0x8eb0)"),
    ('a_direto', "    if direto != 0 {\n        let linha", "    if direto == 0 {\n        let linha"),
    ('a_menos1', "    chamar(FN_MATCH_PLAYER_MOVE_TO, [x as u8 as u32, y as u8 as u32, u32::MAX, 0, 0])", "    chamar(FN_MATCH_PLAYER_MOVE_TO, [x as u8 as u32, y as u8 as u32, 0, 0, 0])"),
]
