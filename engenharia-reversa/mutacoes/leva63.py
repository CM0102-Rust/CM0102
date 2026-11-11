# Mutacoes da leva63 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva63.py --jogo 2001-08-15
MUTACOES = [
    ('h_16', "        if manager != 0 && person == manager && *(person as *const i32) >= read_i32(G_STAFF_COUNT).wrapping_sub(16) {", "        if manager != 0 && person == manager && *(person as *const i32) >= read_i32(G_STAFF_COUNT).wrapping_sub(15) {"),
    ('h_dono', "if manager != 0 && person == manager &&", "if manager != 0 &&"),
    ('h_tatica', "            let tactic = without_tactic == 0\n", "            let tactic = without_tactic != 0\n"),
    ('h_9a01', "Ok(r) if read_u32(r + human_manager_record::ON_HOLIDAY) != 0));", "Ok(r) if read_u32(r + 0x9a05) != 0));"),
    ('h_forcar', "                if force != 0 || any != 0 {", "                if any != 0 {"),
    ('h_selecao', "let yes = if exe::database_is_national_team(club) != 0 {", "let yes = if exe::database_is_national_team(club) == 0 {"),
    ('h_segunda', "        any = 0;\n", ""),
    ('h_volta', "        if exe::club_is_national_team(club) == 0 {\n            return 0;\n        }", "        if exe::club_is_national_team(club) == 0 || true {\n            return 0;\n        }"),
]
