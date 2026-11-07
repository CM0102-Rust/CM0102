# Mutacoes da leva69 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva69.py
MUTACOES = [
    ('c_limite', "    if *(clube as *const i32) >= limite {\n        let continente", "    if *(clube as *const i32) > limite {\n        let continente"),
    ('c_cont0', "        if continente != 0 && (nacao == 0 || d(nacao, 0x71) != continente) {", "        if continente != 0 && (nacao != 0 && d(nacao, 0x71) != continente) {"),
    ('c_divisao', "        if nacao == d(base, 0x5d) && d(clube, 0x57) != 0 && d(clube, 0x57) == base {", "        if nacao == d(base, 0x5d) && d(clube, 0x57) != 0 {"),
    ('c_outro', "        if d(nacao, 0x71) != d(base, 0x59) {\n            return 0;", "        if d(nacao, 0x71) == d(base, 0x59) {\n            return 0;"),
    ('c_grupo', "        if byte(fase, 0x43) == 2 {\n            grupo = fase;", "        if byte(fase, 0x43) == 1 {\n            grupo = fase;"),
    ('c_passo', "if grupo != 0 { (grupo, 0xb1, 0x3b) }", "if grupo != 0 { (grupo, 0xb1, 0x3a) }"),
    ('c_ate', "    while i <= n {", "    while i < n {"),
    ('e_lista', "[KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]\n        .iter()", "[KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES]\n        .iter()"),
]
