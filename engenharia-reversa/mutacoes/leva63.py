# Mutacoes da leva63 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva63.py --jogo 2001-08-15
MUTACOES = [
    ('h_16', "*(pessoa as *const i32) >= (*(0xae2364 as *const i32)).wrapping_sub(16) {", "*(pessoa as *const i32) >= (*(0xae2364 as *const i32)).wrapping_sub(15) {"),
    ('h_dono', "if tecnico != 0 && pessoa == tecnico &&", "if tecnico != 0 &&"),
    ('h_tatica', "            let tatica = sem_tatica == 0\n", "            let tatica = sem_tatica != 0\n"),
    ('h_9a01', "Ok(r) if *((r + 0x9a01) as *const u32) != 0));", "Ok(r) if *((r + 0x9a05) as *const u32) != 0));"),
    ('h_forcar', "                if forcar != 0 || qualquer != 0 {", "                if qualquer != 0 {"),
    ('h_selecao', "let sim = if e_selecao(clube) != 0 {", "let sim = if e_selecao(clube) == 0 {"),
    ('h_segunda', "        qualquer = 0;\n", ""),
    ('h_volta', "        if segunda(clube) == 0 {\n            return 0;\n        }", "        if segunda(clube) == 0 || true {\n            return 0;\n        }"),
]
