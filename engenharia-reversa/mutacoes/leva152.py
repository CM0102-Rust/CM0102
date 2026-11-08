# Mutacoes da leva152 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva152.py --carreira 2002-06-01
MUTACOES = [
    ('j_pendente', "            if byte(p, day_match::HOME_GOALS) == -1 && u(p, day_match::ENGINE) == 0 {", "            if byte(p, day_match::HOME_GOALS) == -1 && u(p, day_match::ENGINE) == 1 {"),
    ('j_fila', "                exe::virtual_staff_fill_fixture_squads(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY, -2, 0);", "                exe::virtual_staff_fill_fixture_squads(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY, -1, 0);"),
    ('j_rapida', "                    exe::cm3_match_engine_init_match(m, p + day_match::FIXTURE_COPY, r, 0, 0);\n                    let rapido", "                    exe::cm3_match_engine_init_match(m, p + day_match::FIXTURE_COPY, 0, 0, 0);\n                    let rapido"),
    ('j_conta', "                *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 1;\n            }\n        }\n    };", "                *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 3;\n            }\n        }\n    };"),
    ('j_fechar', "                    exe::match_day_apply_match_result(day, p);\n", ""),
    ('j_linhas', "                            exe::formation_clear_marking_targets(exe::human_manager_get_active_tactic(manager(), u(u(p, oc), 0xcf)));\n", ""),
    ('j_andar', "        exe::set_cursor_mode(hora);\n        exe::match_day_step_background_match(day, 1);\n", "        exe::set_cursor_mode(hora);\n"),
    ('j_iniciar', "        exe::match_day_start_next_background_match(day);\n        let mut vistos", "        let mut vistos"),
]
