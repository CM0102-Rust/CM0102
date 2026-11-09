# Mutacoes da leva154 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva154.py --carreira 2002-06-01
MUTACOES = [
    ('g_cheio', "        if quantos < 0x9c4 {\n", "        if quantos < 0x10 {\n"),
    ('g_numero', "        numero = *(r as *const i32) + 1;\n        *(r as *mut i32) = numero;", "        numero = *(r as *const i32) + 2;\n        *(r as *mut i32) = numero;"),
    ('g_desc', "    *desc.add(2) = numero as u32;\n", "    *desc.add(2) = 0;\n"),
    ('l_numero', "        if numero as u32 == *desc.add(2) {", "        if numero as u32 != *desc.add(2) {"),
    ('r_gols', "    if engine.score() as i32 > *((summary + match_summary::SIDES) as *const u8) as i32 {", "    if engine.score() as i32 >= *((summary + match_summary::SIDES) as *const u8) as i32 {"),
    ('r_passo', "        let r = (exe::cm3_match_engine_run_match_steps(engine, 1) as u32) as u8;", "        let r = (exe::cm3_match_engine_run_match_steps(engine, 2) as u32) as u8;"),
]
