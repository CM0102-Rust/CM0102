# Mutacoes da leva149 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva149.py --carreira 2002-06-01
MUTACOES = [
    ('e_mando', "        write_u8(t + match_manager::TACTIC_SLOT, 0);\n        write_u8(t + match_manager::SELECTION_MODE, 3);", "        write_u8(t + match_manager::TACTIC_SLOT, 0);\n        write_u8(t + match_manager::SELECTION_MODE, 2);"),
    ('e_mandante', "        write_u8(t + match_manager::TACTIC_SLOT, 1);\n        write_u8(t + match_manager::SELECTION_MODE, 0);", "        write_u8(t + match_manager::TACTIC_SLOT, 1);\n        write_u8(t + match_manager::SELECTION_MODE, 1);"),
    ('e_banco', "    let bench = ((game.squad_rules() as u8) >> 4) & 0xf;", "    let bench = (((game.squad_rules() as u8) >> 4) & 0xf).saturating_sub(1);"),
    ('e_modo', "        n = exe::match_man_pick_team(t, other, 2);", "        n = exe::match_man_pick_team(t, other, 1);"),
    ('e_linhas', "    write_usize(t + match_manager::TACTICS, m.addr() + engine::TACTICS + (side as u8 as i8 as isize * 0x18e3) as usize);", "    write_usize(t + match_manager::TACTICS, m.addr() + engine::TACTICS + ((1 - side as u8 as i8) as isize * 0x18e3) as usize);"),
    ('e_ficha', "        write_u16(t + match_manager::MANAGER_HOME_REPUTATION, (sheet.home_reputation() as u16));", "        write_u16(t + match_manager::MANAGER_HOME_REPUTATION, (sheet.current_reputation() as u16));"),
    ('e_tecnico', "            if id >= base - 0x10 && id < base && read_i8(x + staff::CLASSIFICATION) != 0 {", "            if id >= base - 0x10 && id < base && read_i8(x + staff::CLASSIFICATION) == 0 {"),
    ('e_casa', "        *slots = 0;\n        let total", "        *slots = 1;\n        let total"),
]
