# Mutacoes da leva52 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva52.py
MUTACOES = [
    ('t_ultimo', "*list(byte(j, 0x28) as isize) = *list(*n as isize - 1);", "*list(byte(j, 0x28) as isize) = *list(*n as isize - 2);"),
    ('t_movido', "        *((movido + 0x28) as *mut i8) = idx;", "        *((movido + 0x28) as *mut i8) = idx + 1;"),
    ('t_na', "        if na(*marcado, j) != 0 {", "        if na(*marcado, j) == 0 {"),
    ('t_lista', "            if *((q + match_player::MARKING_TARGET) as *const usize) == j {\n                *((q + match_player::MARKING_TARGET) as *mut usize) = 0;", "            if *((q + match_player::MARKING_TARGET) as *const usize) != 0 {\n                *((q + match_player::MARKING_TARGET) as *mut usize) = 0;"),
    ('t_modo', "    if mode != -3 && j != *((this + 0xf5a2)", "    if mode != -2 && j != *((this + 0xf5a2)"),
    ('t_atac', "    if match_player_position(j) & 0x40 != 0 || match_player_position(j) & 0x20 != 0 {", "    if match_player_position(j) & 0x40 != 0 {"),
    ('t_904b', "            if *((block_manager + 0x904b) as *const u8) == 0 {", "            if *((block_manager + 0x904b) as *const u8) != 0 {"),
    ('t_sel', "            if national_team(club) != 0 {", "            if national_team(club) == 0 {"),
    ('t_faixa', "if id >= npes - 16 && id < npes &&", "if id >= npes - 15 && id < npes &&"),
    ('t_filtro', "if chosen != 0 && (match_player_position(q) & 1 != 0 || match_player_position(q) & 4 != 0 || match_player_position(q) & 2 != 0)", "if chosen != 0 && (match_player_position(q) & 1 != 0 || match_player_position(q) & 4 != 0)"),
    ('t_15000', "        } else if r & 0x10 != 0 {\n            15000", "        } else if r & 0x10 != 0 {\n            20000"),
    ('t_lados', "        if r & 0x80 != 0 || r & 0x800 != 0 {", "        if r & 0x80 != 0 {"),
    ('t_maior', "        if v > best || chosen == 0 {", "        if v >= best || chosen == 0 {"),
    ('t_goleiro', "    if match_player_position(j) == 1 {\n        let field = if side_j == 0", "    if match_player_position(j) & 1 != 0 {\n        let field = if side_j == 0"),
]
