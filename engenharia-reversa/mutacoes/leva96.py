# Mutacoes da leva96 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva96.py --jogo 2001-08-20
MUTACOES = [
    ('m_infiltra', "                        corre(call, l, 2);", "                        corre(call, l, 3);"),
    ('m_goleiro_9', "                let gy = |gk: MatchPlayer| if !gk.is_null() { gk.cell_y() } else { 9 };", "                let gy = |gk: MatchPlayer| if !gk.is_null() { gk.cell_y() } else { 8 };"),
    ('m_limiar', "            let mut f = f64::from_bits(0x3ff8_0000_0000_0000);", "            let mut f = f64::from_bits(0x3ff4_0000_0000_0000);"),
    ('m_8000', "            pressiona = mk & 0x8000 != 0", "            pressiona = mk & 0x4000 != 0"),
    ('m_71', "        *w(match_player::ACTION) = 0x71;", "        *w(match_player::ACTION) = 0x72;"),
    ('m_78', "            *w(match_player::ACTION) = 0x78;", "            *w(match_player::ACTION) = 0x79;"),
    ('m_2113', "if near_goal { event::LOOSE_BALL_IN_AREA } else { event::PICK_UP_FREE_BALL }", "if near_goal { 0x1f46 } else { 0x1f46 }"),
    ('m_104', "        if other != 0 && my == 1 && b(match_player::MOMENT_POSITIONING) != 0 {", "        if other != 0 && my == 2 && b(match_player::MOMENT_POSITIONING) != 0 {"),
    ('m_76', "        *w(match_player::ACTION) = if pressiona { 0x100 } else { 0x76 };", "        *w(match_player::ACTION) = if pressiona { 0x100 } else { 0x77 };"),
    ('m_largar', "            if pm == -5 || (pm > -4 && pm < 0) {", "            if pm == -5 || (pm > -3 && pm < 0) {"),
    # sobreviventes da leva 95, refeitas com a foto de antes da chamada
    ('s_fica', "            *w(match_player::ACTION) = 0x76;", "            *w(match_player::ACTION) = 0x77;"),
    ('s_formacao', "        *w(match_player::ACTION) = 0x68;\n        call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, [x as u32", "        *w(match_player::ACTION) = 0x69;\n        call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, [x as u32"),
]
