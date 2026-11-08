# Mutacoes da leva63 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva63.py --jogo 2001-08-15
MUTACOES = [
    ('h_16', "*(person as *const i32) >= (*(G_STAFF_COUNT as *const i32)).wrapping_sub(16) {", "*(person as *const i32) >= (*(G_STAFF_COUNT as *const i32)).wrapping_sub(15) {"),
    ('h_dono', "if manager != 0 && person == manager &&", "if manager != 0 &&"),
    ('h_tatica', "            let tactic = without_tactic == 0\n", "            let tactic = without_tactic != 0\n"),
    ('h_9a01', "Ok(r) if *((r + 0x9a01) as *const u32) != 0));", "Ok(r) if *((r + 0x9a05) as *const u32) != 0));"),
    ('h_forcar', "                if force != 0 || qualquer != 0 {", "                if qualquer != 0 {"),
    ('h_selecao', "let sim = if is_national_team(clube) != 0 {", "let sim = if is_national_team(clube) == 0 {"),
    ('h_segunda', "        qualquer = 0;\n", ""),
    ('h_volta', "        if second(clube) == 0 {\n            return 0;\n        }", "        if second(clube) == 0 || true {\n            return 0;\n        }"),
]
