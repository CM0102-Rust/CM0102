# Mutacoes da leva151 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva151.py --carreira 2002-06-01
MUTACOES = [
    ('i_decidida', "            if read_i8(p + day_match::HOME_GOALS) != -2\n", "            if read_i8(p + day_match::HOME_GOALS) != -3\n"),
    ('i_volta', "        *((dia + 0x33) as *mut u8) = 2;\n        por(0x2f, -1);", "        *((dia + 0x33) as *mut u8) = 2;\n        por(0x2f, 0);"),
    ('i_rapida', "                    exe::cm3_match_engine_init_match(engine, p + day_match::FIXTURE_COPY, r, 0, 0);", "                    exe::cm3_match_engine_init_match(engine, p + day_match::FIXTURE_COPY, 1, 0, 0);"),
    ('a_passo', "        step_towards(1);\n", "        step_towards(2);\n"),
    ('a_conta', "            *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 1;", "            *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 2;"),
    ('a_fim', "    let end = |e: u8| e == 2 || e == 8;", "    let end = |e: u8| e == 2;"),
    ('f_copia', "    core::ptr::copy((p + day_match::FIXTURE_COPY) as *const u8, game.addr() as *mut u8, 0x4f);", "    core::ptr::copy((p + day_match::FIXTURE_COPY) as *const u8, game.addr() as *mut u8, 0x4e);"),
    ('f_rodada', "            if !r.is_null() && (r.losing_run() as i32) > 1 {", "            if !r.is_null() && (r.losing_run() as i32) > 0 {"),
    ('f_contratos', "    two(u(G_CONTRACT_MANAGER, 0), 0x4ca6d0);\n", ""),
    ('f_suspensoes', "    two(u(G_DISCIPLINE, 0), 0x55ca00);\n", ""),
    ('f_estados', "    two(u(G_INJURY, 0), 0x62c080);\n", ""),
    ('f_historia', "        two(h, u(u(h, 0), 4));\n", ""),
    ('f_lados', "    club(u(game.addr(), fixture::AWAY_CLUB), u(game.addr(), fixture::HOME_CLUB), game.addr(), round[1] as u32, summary);", "    club(u(game.addr(), fixture::AWAY_CLUB), u(game.addr(), fixture::HOME_CLUB), game.addr(), round[0] as u32, summary);"),
    ('f_metodo', "            metodo(c, game.addr(), summary);", "            let _ = (metodo, c);"),
]
