# Mutacoes da leva152 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva152.py --carreira 2002-06-01
MUTACOES = [
    ('j_pendente', "            if byte(p, day_match::HOME_GOALS) == -1 && u(p, day_match::ENGINE) == 0 {", "            if byte(p, day_match::HOME_GOALS) == -1 && u(p, day_match::ENGINE) == 1 {"),
    ('j_fila', "                remove_of_queue(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY, -2, 0);", "                remove_of_queue(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY, -1, 0);"),
    ('j_rapida', "                    prepare(m, p + day_match::FIXTURE_COPY, r, 0, 0);\n                    let rapido", "                    prepare(m, p + day_match::FIXTURE_COPY, 0, 0, 0);\n                    let rapido"),
    ('j_conta', "                *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 1;\n            }\n        }\n    };", "                *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 3;\n            }\n        }\n    };"),
    ('j_fechar', "                    close(day, p);\n", ""),
    ('j_linhas', "                            clear(guardadas(manager(), u(u(p, oc), 0xcf)));\n", ""),
    ('j_andar', "        adjust_clock(hora);\n        andar(day, 1);\n", "        adjust_clock(hora);\n"),
    ('j_iniciar', "        iniciar(day);\n        let mut vistos", "        let mut vistos"),
]
