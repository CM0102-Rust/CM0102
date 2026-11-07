# Mutacoes da leva133 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva133.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('s_flag', "    if g(0xdd7118) == 0 {\n        return 1;", "    if g(0xdd7118) == 0 {\n        return 0;"),
    ('s_humano', "humano(c1, 1, 1, 0, 0) != 0 || humano(c2, 1, 1, 0, 0) != 0 {\n        return 0;", "humano(c1, 1, 0, 0, 0) != 0 || humano(c2, 1, 1, 0, 0) != 0 {\n        return 0;"),
    ('s_ligado', "humano(ligado(c, 0, 1), 1, 1, 0, 0) != 0", "humano(ligado(c, 0, 1), 1, 0, 0, 0) != 0", "sem humano clube ligado"),
    ('s_comp1', "if comp == ler(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) ||", "if comp == ler(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) ||"),
    ('s_958', "k as isize * 0x122 + 0x11c", "k as isize * 0x122 + 0x11d"),
    ('s_684', "if comp == ler(KEY_CLUB_COMP_RESERVE) && humano(c1", "if comp == ler(KEY_CLUB_COMP_ARGENTINE_THIRD_DIVISION) && humano(c1", "sem humano competicao direta"),
    ('s_16', "for k in 0..16isize {", "for k in 0..15isize {", "sem humano tecnico da nacao"),
    ('s_5f', "*((pe + 0x5f) as *const u8) == 1", "*((pe + 0x5f) as *const u8) == 0", "sem humano tecnico da nacao"),
    ('s_80', "> 0x2ee0 { 0 } else { final_() }", "> 0x2000 { 0 } else { final_() }", "sem humano pela reputacao"),
    ('s_vaga', "if x == 0 || x == 1 {", "if x == 0 || x == 2 {"),
    ('s_marcada', "if matches!(x, 3..=7) {", "if matches!(x, 4..=7) {", "sem humano selecao marcada"),
    ('s_final', "(f7f9720(g(0xb74448), partida) == 0) as u32", "(f7f9720(g(0xb74448), partida) != 0) as u32"),
]
