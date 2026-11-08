# Mutacoes da leva152 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva152.py --carreira 2002-06-01
MUTACOES = [
    ('j_pendente', "            if byte(p, 0x47) == -1 && u(p, 0x57) == 0 {", "            if byte(p, 0x47) == -1 && u(p, 0x57) == 1 {"),
    ('j_fila', "                remove_of_queue(u(G_VIRTUAL_STAFF, 0), p + 4, -2, 0);", "                remove_of_queue(u(G_VIRTUAL_STAFF, 0), p + 4, -1, 0);"),
    ('j_rapida', "                    prepare(m, p + 4, r, 0, 0);\n                    let rapido", "                    prepare(m, p + 4, 0, 0, 0);\n                    let rapido"),
    ('j_conta', "                *((day + 0x25) as *mut i32) += 1;\n            }\n        }\n    };", "                *((day + 0x25) as *mut i32) += 3;\n            }\n        }\n    };"),
    ('j_fechar', "                    close(day, p);\n", ""),
    ('j_linhas', "                            clear(guardadas(manager(), u(u(p, oc), 0xcf)));\n", ""),
    ('j_andar', "        ajustar_relogio(hora);\n        andar(day, 1);\n", "        ajustar_relogio(hora);\n"),
    ('j_iniciar', "        iniciar(day);\n        let mut vistos", "        let mut vistos"),
]
