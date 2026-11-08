# Mutacoes da leva151 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva151.py --carreira 2002-06-01
MUTACOES = [
    ('i_decidida', "            if byte(p, 0x47) != -2\n", "            if byte(p, 0x47) != -3\n"),
    ('i_volta', "        *((dia + 0x33) as *mut u8) = 2;\n        por(0x2f, -1);", "        *((dia + 0x33) as *mut u8) = 2;\n        por(0x2f, 0);"),
    ('i_rapida', "                    preparar(motor, p + 4, r, 0, 0);", "                    preparar(motor, p + 4, 1, 0, 0);"),
    ('a_passo', "        step_towards(1);\n", "        step_towards(2);\n"),
    ('a_conta', "            *((dia + 0x25) as *mut i32) += 1;", "            *((dia + 0x25) as *mut i32) += 2;"),
    ('a_fim', "    let fim = |e: u8| e == 2 || e == 8;", "    let fim = |e: u8| e == 2;"),
    ('f_copia', "    core::ptr::copy((p + 4) as *const u8, jogo as *mut u8, 0x4f);", "    core::ptr::copy((p + 4) as *const u8, jogo as *mut u8, 0x4e);"),
    ('f_rodada', "            if r != 0 && *((r + 0x2d) as *const i32) > 1 {", "            if r != 0 && *((r + 0x2d) as *const i32) > 0 {"),
    ('f_contratos', "    dois(u(G_CONTRACT_MANAGER, 0), 0x4ca6d0);\n", ""),
    ('f_suspensoes', "    dois(u(G_DISCIPLINE, 0), 0x55ca00);\n", ""),
    ('f_estados', "    dois(u(G_INJURY, 0), 0x62c080);\n", ""),
    ('f_historia', "        dois(h, u(u(h, 0), 4));\n", ""),
    ('f_lados', "    clube(u(jogo, 0x20), u(jogo, 0x1c), jogo, rodada[1] as u32, resumo);", "    clube(u(jogo, 0x20), u(jogo, 0x1c), jogo, rodada[0] as u32, resumo);"),
    ('f_metodo', "            metodo(c, jogo, resumo);", "            let _ = (metodo, c);"),
]
