# Mutacoes da leva149 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva149.py --carreira 2002-06-01
MUTACOES = [
    ('e_mando', "        *b(match_manager::TACTIC_SLOT) = 0;\n        *b(match_manager::SELECTION_MODE) = 3;", "        *b(match_manager::TACTIC_SLOT) = 0;\n        *b(match_manager::SELECTION_MODE) = 2;"),
    ('e_mandante', "        *b(match_manager::TACTIC_SLOT) = 1;\n        *b(match_manager::SELECTION_MODE) = 0;", "        *b(match_manager::TACTIC_SLOT) = 1;\n        *b(match_manager::SELECTION_MODE) = 1;"),
    ('e_banco', "    let bench = (*((game + fixture::SQUAD_RULES) as *const u8) >> 4) & 0xf;", "    let bench = ((*((game + fixture::SQUAD_RULES) as *const u8) >> 4) & 0xf).saturating_sub(1);"),
    ('e_modo', "        n = exe::match_man_pick_team(t, other, 2);", "        n = exe::match_man_pick_team(t, other, 1);"),
    ('e_linhas', "    *d(match_manager::TACTICS) = m + engine::TACTICS + (side as u8 as i8 as isize * 0x18e3) as usize;", "    *d(match_manager::TACTICS) = m + engine::TACTICS + ((1 - side as u8 as i8) as isize * 0x18e3) as usize;"),
    ('e_ficha', "        *((t + match_manager::MANAGER_HOME_REPUTATION) as *mut u16) = *((sheet + non_player::HOME_REPUTATION) as *const u16);", "        *((t + match_manager::MANAGER_HOME_REPUTATION) as *mut u16) = *((sheet + non_player::CURRENT_REPUTATION) as *const u16);"),
    ('e_tecnico', "            if id >= base - 0x10 && id < base && byte(x, staff::CLASSIFICATION) != 0 {", "            if id >= base - 0x10 && id < base && byte(x, staff::CLASSIFICATION) == 0 {"),
    ('e_casa', "        *slots = 0;\n        let total", "        *slots = 1;\n        let total"),
]
