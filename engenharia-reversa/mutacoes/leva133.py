# Mutacoes da leva133 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva133.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('s_flag', "    if g(G_INTERACTIVE_MODE_FLAG) == 0 {\n        return 1;", "    if g(G_INTERACTIVE_MODE_FLAG) == 0 {\n        return 0;"),
    ('s_humano', "humano(c1, 1, 1, 0, 0) != 0 || humano(c2, 1, 1, 0, 0) != 0 {\n        return 0;", "humano(c1, 1, 0, 0, 0) != 0 || humano(c2, 1, 1, 0, 0) != 0 {\n        return 0;"),
    ('s_ligado', "human(exe::club_get_linked_b_team(c, (0) as *mut u32, 1), 1, 1, 0, 0) != 0", "human(exe::club_get_linked_b_team(c, (0) as *mut u32, 1), 1, 0, 0, 0) != 0", "no human linked club"),
    ('s_comp1', "if comp == read(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) ||", "if comp == read(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) ||"),
    ('s_958', "k as isize * 0x122 + (nation::LEAGUE_SELECTED as isize)", "k as isize * 0x122 + 0x11d"),
    ('s_684', "if comp == read(KEY_CLUB_COMP_RESERVE) && human(c1", "if comp == read(KEY_CLUB_COMP_ARGENTINE_THIRD_DIVISION) && human(c1", "no human direct competition"),
    ('s_16', "for k in 0..16isize {", "for k in 0..15isize {", "no human nation manager"),
    ('s_5f', "pe.classification() == 1", "pe.classification() == 0", "no human nation manager"),
    ('s_80', "> 0x2ee0 { 0 } else { final_() }", "> 0x2000 { 0 } else { final_() }", "no human by injury_word_0c"),
    ('s_vaga', "if x == 0 || x == 1 {", "if x == 0 || x == 2 {"),
    ('s_marcada', "if matches!(x, 3..=7) {", "if matches!(x, 4..=7) {", "no human national team marked"),
    ('s_final', "(f7f9720(g(0xb74448), partida) == 0) as u32", "(f7f9720(g(0xb74448), partida) != 0) as u32"),
]
