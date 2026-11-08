# Mutacoes da leva152 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva152.py --carreira 2002-06-01
MUTACOES = [
    ('j_pendente', "            if byte(p, 0x47) == -1 && u(p, 0x57) == 0 {", "            if byte(p, 0x47) == -1 && u(p, 0x57) == 1 {"),
    ('j_fila', "                tirar_da_fila(u(G_VIRTUAL_STAFF, 0), p + 4, -2, 0);", "                tirar_da_fila(u(G_VIRTUAL_STAFF, 0), p + 4, -1, 0);"),
    ('j_rapida', "                    preparar(m, p + 4, r, 0, 0);\n                    let rapido", "                    preparar(m, p + 4, 0, 0, 0);\n                    let rapido"),
    ('j_conta', "                *((dia + 0x25) as *mut i32) += 1;\n            }\n        }\n    };", "                *((dia + 0x25) as *mut i32) += 3;\n            }\n        }\n    };"),
    ('j_fechar', "                    fechar(dia, p);\n", ""),
    ('j_linhas', "                            limpar(guardadas(gerente(), u(u(p, oc), 0xcf)));\n", ""),
    ('j_andar', "        ajustar_relogio(hora);\n        andar(dia, 1);\n", "        ajustar_relogio(hora);\n"),
    ('j_iniciar', "        iniciar(dia);\n        let mut vistos", "        let mut vistos"),
]
